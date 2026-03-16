use aya::Pod;

#[repr(C)]
#[derive(Copy, Clone, Pod)]
pub struct GlrdEventRaw {
    pub pid: u32,
    pub nombre_proceso: [u8; 255],
    pub ruta_ejecutable: [u8; 512],
    pub usuario_so: [u8; 64],
    pub ruta_ejemplo: [u8; 512],
    pub archivos_afectados: u32,
    pub entropia_x10000: u32,
    pub timestamp_ns: u64,
}

pub struct GlrdEvent {
    pub pid: u32,
    pub nombre_proceso: String,
    pub ruta_ejecutable: String,
    pub usuario_so: String,
    pub ruta_ejemplo: String,
    pub archivos_afectados: u32,
    pub entropia: f64,
    pub timestamp_ns: u64,
}

impl GlrdEvent {
    pub fn from_raw(raw: &GlrdEventRaw) -> Self {
        Self {
            pid: raw.pid,
            nombre_proceso: cstr_to_string(&raw.nombre_proceso),
            ruta_ejecutable: cstr_to_string(&raw.ruta_ejecutable),
            usuario_so: cstr_to_string(&raw.usuario_so),
            ruta_ejemplo: cstr_to_string(&raw.ruta_ejemplo),
            archivos_afectados: raw.archivos_afectados,
            entropia: raw.entropia_x10000 as f64 / 10_000.0,
            timestamp_ns: raw.timestamp_ns,
        }
    }
}

fn cstr_to_string(buf: &[u8]) -> String {
    let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
    String::from_utf8_lossy(&buf[..end]).into_owned()
}
