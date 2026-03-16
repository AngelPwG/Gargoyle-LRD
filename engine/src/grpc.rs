pub mod proto {
    tonic::include_proto!("glrd");
}

use proto::{
    glrd_service_client::GlrdServiceClient,
    ArchivoAfectado, Incidente,
};
use tonic::transport::Channel;

use crate::event::GlrdEvent;

pub struct GlrdClient {
    inner: GlrdServiceClient<Channel>,
}

impl GlrdClient {
    pub async fn connect(addr: &str) -> anyhow::Result<Self> {
        let inner = GlrdServiceClient::connect(addr.to_owned()).await?;
        Ok(Self { inner })
    }

    pub async fn reportar(&mut self, event: &GlrdEvent, archivos: Vec<ArchivoAfectado>) -> anyhow::Result<()> {
        let req = Incidente {
            pid:                  event.pid,
            nombre_proceso:       event.nombre_proceso.clone(),
            ruta_ejecutable:      event.ruta_ejecutable.clone(),
            usuario_so:           event.usuario_so.clone(),
            entropia_promedio:    event.entropia,
            archivos_afectados:   event.archivos_afectados,
            ruta_ejemplo:         event.ruta_ejemplo.clone(),
            accion_tomada:        String::new(), // se llena después del HITL
            timestamp_deteccion:  format_ts(event.timestamp_ns),
            timestamp_resolucion: String::new(), // se llena después del kill
            archivos,
        };

        let resp = self.inner.reportar_incidente(tonic::Request::new(req)).await?;

        if !resp.into_inner().ok {
            anyhow::bail!("Spring Boot rechazó el incidente");
        }

        Ok(())
    }
}

fn format_ts(ns: u64) -> String {
    // Convierte timestamp_ns a ISO-8601 básico
    // Por ahora divide a segundos — mejorar con chrono si hace falta
    let secs = ns / 1_000_000_000;
    let dt = std::time::UNIX_EPOCH + std::time::Duration::from_secs(secs);
    // chrono lo hace limpio, pero sin dependencia extra:
    format!("{:?}", dt)
}
