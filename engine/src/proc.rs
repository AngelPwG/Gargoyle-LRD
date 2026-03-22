use std::fs;
use std::os::unix::fs::MetadataExt;

use crate::grpc::proto::ArchivoAfectado;

/// Lee /proc/<pid>/fd/ y construye la lista de archivos
/// regulares abiertos por el proceso en este momento.
///
/// Limitación conocida: solo captura fds abiertos en el instante
/// de la llamada. Archivos ya cerrados no aparecen.
/// Por eso ruta_ejemplo del kernel sigue siendo el dato primario.
pub fn leer_archivos_abiertos(pid: u32) -> Vec<ArchivoAfectado> {
    let fd_dir = format!("/proc/{}/fd", pid);

    let entries = match fs::read_dir(&fd_dir) {
        Ok(e) => e,
        Err(_) => return vec![], // proceso ya terminó o sin permisos
    };

    entries
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            // Cada fd es un symlink → resolver a la ruta real
            let ruta = fs::read_link(entry.path()).ok()?;
            let ruta_str = ruta.to_string_lossy().to_string();

            // Filtrar: solo archivos regulares en el filesystem
            // Descartar sockets, pipes, anon_inode, etc.
            if !ruta_str.starts_with('/') {
                return None;
            }
            if ruta_str.contains("socket:")
                || ruta_str.contains("pipe:")
                || ruta_str.contains("anon_inode")
            {
                return None;
            }

            // Obtener tamaño del archivo via stat
            let bytes = fs::metadata(&ruta).map(|m| m.size() as i64).unwrap_or(0);

            // Extraer extensión
            let ext = ruta
                .extension()
                .map(|e| format!(".{}", e.to_string_lossy()))
                .unwrap_or_default();

            Some(ArchivoAfectado {
                ruta: ruta_str,
                ext,
                bytes,
            })
        })
        .collect()
}
