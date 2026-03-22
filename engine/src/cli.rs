use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "glrd", about = "Gargoyle Linux Ransomware Detector")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Arranca el motor de detección
    Start {
        /// Override del umbral de entropía (default: leer de MySQL)
        #[arg(long, value_name = "FLOAT")]
        umbral: Option<f64>,

        /// URL de conexión a MySQL
        #[arg(long, env = "GLRD_DB_URL",
              default_value = "mysql://spring_backend:password@localhost/glrd")]
        db_url: String,

        /// Ruta del buffer local sled
        #[arg(long, default_value = "/var/lib/glrd/buffer")]
        buffer: String,
    },
}
