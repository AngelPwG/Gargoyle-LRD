pub mod proto {
    tonic::include_proto!("glrd");
}

use proto::{
    glrd_service_client::GlrdServiceClient,
    ArchivoAfectado, Incidente,
};
use tonic::transport::Channel;

use crate::event::GlrdEvent;

#[derive(Clone)]
pub struct GlrdClient {
    inner: GlrdServiceClient<Channel>,
}

impl GlrdClient {
    pub async fn connect(addr: &str) -> anyhow::Result<Self> {
        let inner = GlrdServiceClient::connect(addr.to_owned()).await?;
        Ok(Self { inner })
    }

    pub async fn reportar(
        &mut self,
        event: &GlrdEvent,
        archivos: Vec<ArchivoAfectado>,
    ) -> anyhow::Result<()> {
        let req = construir_incidente(event, archivos);

        let resp = self.inner
            .reportar_incidente(tonic::Request::new(req))
            .await?;

        if !resp.into_inner().ok {
            anyhow::bail!("Spring Boot rechazó el incidente");
        }

        Ok(())
    }
}

fn construir_incidente(event: &GlrdEvent, archivos: Vec<ArchivoAfectado>) -> Incidente {
    Incidente {
        pid:                  event.pid,
        nombre_proceso:       event.nombre_proceso.clone(),
        ruta_ejecutable:      event.ruta_ejecutable.clone(),
        usuario_so:           event.usuario_so.clone(),
        entropia_promedio:    event.entropia,
        archivos_afectados:   event.archivos_afectados,
        ruta_ejemplo:         event.ruta_ejemplo.clone(),
        accion_tomada:        event.accion_tomada.clone(),
        timestamp_deteccion:  ns_to_iso8601(event.timestamp_ns),
        timestamp_resolucion: event.timestamp_resolucion.clone(),
        archivos,
    }
}

/// Convierte timestamp en nanosegundos (CLOCK_REALTIME del kernel)
/// a string ISO-8601 que Jorge espera en el payload.
fn ns_to_iso8601(ns: u64) -> String {
    use chrono::{DateTime, Utc, TimeZone};
    let secs  = (ns / 1_000_000_000) as i64;
    let nanos = (ns % 1_000_000_000) as u32;
    match Utc.timestamp_opt(secs, nanos) {
        chrono::LocalResult::Single(dt) => dt.format("%Y-%m-%dT%H:%M:%S").to_string(),
        _ => format!("ts_invalido:{}", ns),
    }
}
