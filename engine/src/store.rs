use sled::Db;
use crate::event::GlrdEvent;

pub struct Store {
    db: Db,
}

impl Store {
    pub fn open(path: &str) -> anyhow::Result<Self> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }

    /// Persiste el evento antes de enviarlo a Spring Boot.
    /// Clave: timestamp_ns como big-endian bytes (ordenación cronológica).
    /// Valor: JSON del evento serializado.
    pub fn insertar(&self, event: &GlrdEvent) -> anyhow::Result<()> {
        let key = event.timestamp_ns.to_be_bytes();
        let value = serde_json::to_vec(event)?;
        self.db.insert(key, value)?;
        Ok(())
    }

    /// Elimina el evento del buffer una vez confirmado
    /// que Spring Boot lo recibió correctamente.
    pub fn confirmar(&self, timestamp_ns: u64) -> anyhow::Result<()> {
        let key = timestamp_ns.to_be_bytes();
        self.db.remove(key)?;
        Ok(())
    }
}
