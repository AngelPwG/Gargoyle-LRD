use std::io::{self, Write};
use std::process::Command;
use std::time::SystemTime;

use chrono::{DateTime, Utc};

use crate::event::GlrdEvent;
use crate::grpc::proto::ArchivoAfectado;

pub enum Decision {
    Kill { timestamp_resolucion: String },
    Descartar,
}

/// Muestra la alerta al operador, espera confirmación y ejecuta
/// la acción correspondiente.
///
/// Retorna Decision::Kill si el operador confirmó,
/// Decision::Descartar si rechazó o no respondió con 's'.
pub fn ejecutar(event: &GlrdEvent, archivos: &[ArchivoAfectado]) -> Decision {
    imprimir_alerta(event, archivos);

    print!("\n¿Terminar proceso? [s/N]: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or(0);

    if input.trim().eq_ignore_ascii_case("s") {
        match matar_proceso(event.pid) {
            Ok(ms) => {
                let ts = timestamp_iso8601();
                println!("[OK] kill -SIGKILL {} ejecutado en {:.1}ms", event.pid, ms);
                Decision::Kill { timestamp_resolucion: ts }
            }
            Err(e) => {
                eprintln!("[ERROR] No se pudo matar el proceso {}: {}", event.pid, e);
                Decision::Descartar
            }
        }
    } else {
        println!("[DESCARTADO] Proceso {} ignorado por el operador.", event.pid);
        Decision::Descartar
    }
}

fn imprimir_alerta(event: &GlrdEvent, archivos: &[ArchivoAfectado]) {
    println!("\n{}", "─".repeat(60));
    println!("  [ALERTA GARGOYLE LRD]");
    println!("{}", "─".repeat(60));
    println!("  PID            : {}", event.pid);
    println!("  Proceso        : {}", event.nombre_proceso);
    println!("  Ejecutable     : {}", event.ruta_ejecutable);
    println!("  Usuario        : {}", event.usuario_so);
    println!("  Entropía       : {:.4}  (umbral superado)", event.entropia);
    println!("  Archivos afect.: {}", event.archivos_afectados);
    println!("  Último archivo : {}", event.ruta_ejemplo);
    println!("  Timestamp      : {}", event.timestamp_ns);

    if !archivos.is_empty() {
        println!("\n  Archivos abiertos ahora:");
        for a in archivos.iter().take(5) {
            println!("    {} ({} bytes)", a.ruta, a.bytes);
        }
        if archivos.len() > 5 {
            println!("    ... y {} más", archivos.len() - 5);
        }
    }
    println!("{}", "─".repeat(60));
}

fn matar_proceso(pid: u32) -> anyhow::Result<f64> {
    let inicio = SystemTime::now();

    let status = Command::new("kill")
        .args(["-SIGKILL", &pid.to_string()])
        .status()?;

    let ms = inicio.elapsed()?.as_secs_f64() * 1000.0;

    if status.success() {
        Ok(ms)
    } else {
        anyhow::bail!("kill retornó código no-cero: {:?}", status.code())
    }
}

fn timestamp_iso8601() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.format("%Y-%m-%dT%H:%M:%S").to_string()
}
