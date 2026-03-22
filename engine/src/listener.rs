use crate::engine::Engine;
use crate::proc::leer_archivos_abiertos;
use crate::hitl;
use crate::store::Store;
use crate::grpc::GlrdClient;
use aya::maps::perf::AsyncPerfEventArray;
use aya::util::online_cpus;
use aya::Ebpf;
use prost::bytes::BytesMut;
use tokio::task;
use std::sync::Arc;
use crate::event::{GlrdEvent, GlrdEventRaw};
use crate::user::resolve_username;
use crate::grpc::proto::ArchivoAfectado;

pub async fn run(
    bpf: &mut Ebpf,
    engine: Arc<Engine>,
    store: Arc<Store>,
    grpc: GlrdClient,
) -> anyhow::Result<()> {

    let mut perf_array = AsyncPerfEventArray::try_from(
        bpf.take_map("GLRD_EVENTS").expect("mapa GLRD_EVENTS no encontrado"),
    )?;

    for cpu_id in online_cpus().map_err(|(_, e)| e)? {
        let mut buf = perf_array.open(cpu_id, None)?;
        let engine = Arc::clone(&engine);
        let store = Arc::clone(&store);
        let grpc = grpc.clone();

        task::spawn(async move {
            let mut buffers = vec![BytesMut::with_capacity(
                std::mem::size_of::<GlrdEventRaw>(),
            )];

            loop {
                let events = buf.read_events(&mut buffers).await.unwrap();

                for i in 0..events.read {
                    let raw: GlrdEventRaw = unsafe {
                        std::ptr::read(buffers[i].as_ptr() as *const GlrdEventRaw)
                    };

                    let mut event = GlrdEvent::from_raw(&raw);

                    // Resolver usuario independientemente de si llegó como
                    // nombre o como UID numérico
                    event.usuario_so = resolve_username(&event.usuario_so);

                    if engine.es_sospechoso(&event) {
                        let archivos = leer_archivos_abiertos(event.pid);

                        if let Err(e) = store.insertar(&event) {
                            eprintln!("sled error: {}", e);
                        }
                        handle_alerta(event, archivos, Arc::clone(&store), grpc.clone()).await;
                    }
                }
            }
        });
    }

    Ok(())
}

async fn handle_alerta(
    mut event: GlrdEvent,
    archivos: Vec<ArchivoAfectado>,
    store: Arc<Store>,
    mut grpc: GlrdClient,
) {
    let event_clone = event.clone();
    let archivos_clone = archivos.clone();

    let decision = tokio::task::spawn_blocking(move || {
        hitl::ejecutar(&event_clone, &archivos_clone)
    })
    .await
    .unwrap_or(hitl::Decision::Descartar);

    match decision {
        hitl::Decision::Kill { timestamp_resolucion } => {
            event.accion_tomada = "KILL".to_string();
            event.timestamp_resolucion = timestamp_resolucion;

            if let Err(e) = store.insertar(&event) {
                eprintln!("sled update error: {}", e);
            }

            match grpc.reportar(&event, archivos).await {
                Ok(_) => {
                    println!("[gRPC] incidente enviado a Spring Boot");
                    let _ = store.confirmar(event.timestamp_ns);
                }
                Err(e) => {
                    eprintln!("[gRPC] error al enviar: {} - evento en buffer sled", e);
                }
            }
        }
        hitl::Decision::Descartar => {
            event.accion_tomada = "IGNORED".to_string();
            let _ = store.confirmar(event.timestamp_ns);
            println!("[HITL] evento descartado por operador");
        }
    }
}
