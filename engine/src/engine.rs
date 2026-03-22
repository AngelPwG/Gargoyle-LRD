use crate::event::GlrdEvent;

pub struct Engine {
    pub umbral: f64,
}

impl Engine {
    pub fn new(umbral: f64) -> Self {
        Self { umbral }
    }

    /// Retorna true si el evento supera el umbral y debe
    /// disparar el loop HITL.
    pub fn es_sospechoso(&self, event: &GlrdEvent) -> bool {
        event.entropia >= self.umbral
    }
}
