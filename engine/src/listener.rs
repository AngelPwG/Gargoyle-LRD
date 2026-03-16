// src/listener.rs

use aya::maps::perf::AsyncPerfEventArray;
use aya::util::online_cpus;
use aya::Ebpf;
use bytes::BytesMut;
use tokio::task;

use crate::event::{GlrdEvent, GlrdEventRaw};

pub async fn run(bpf: &mut Ebpf) -> anyhow::Result<()> {
    let mut perf_array = AsyncPerfEventArray::try_from(
        bpf.map_mut("GLRD_EVENTS").expect("mapa GLRD_EVENTS no encontrado"),
    )?;

    for cpu_id in online_cpus()? {
        let mut buf = perf_array.open(cpu_id, None)?;

        task::spawn(async move {
            // Un buffer por CPU — aya entrega un evento por poll
            let mut buffers = vec![BytesMut::with_capacity(
                std::mem::size_of::<GlrdEventRaw>(),
            )];

            loop {
                let events = buf.read_events(&mut buffers).await.unwrap();

                for i in 0..events.read {
                    let raw: GlrdEventRaw =
                        *aya::Pod::from_bytes(&buffers[i]).unwrap();

                    let event = GlrdEvent::from_raw(&raw);
                    handle_event(event).await;
                }
            }
        });
    }

    Ok(())
}

async fn handle_event(event: GlrdEvent) {
    // Por ahora solo imprimimos — aquí irá la lógica de entropía + HITL
    println!(
        "[{}] pid={} proceso={} entropía={:.4} archivos={}",
        event.timestamp_ns,
        event.pid,
        event.nombre_proceso,
        event.entropia,
        event.archivos_afectados,
    );
}
