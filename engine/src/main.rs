mod cli;
mod db;
mod engine;
mod event;
mod grpc;
mod hitl;
mod listener;
mod proc;
mod store;
mod user;
use clap::Parser;
use cli::{Cli, Commands};
use std::sync::Arc;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Start { umbral, db_url, buffer } => {
            // 1. MySQL → umbral
            let (_pool, db_umbral) = db::init(&db_url).await?;
            let umbral_valor = umbral.unwrap_or(db_umbral.valor);
            println!("[OK] umbral de entropía: {:.4}", umbral_valor);

            // 2. Engine + Store
            let engine = Arc::new(engine::Engine::new(umbral_valor));
            let store  = Arc::new(store::Store::open(&buffer)?);

            // 3. Cliente gRPC
            let grpc = grpc::GlrdClient::connect("http://localhost:50051").await?;

            // 4. Cargar eBPF y arrancar
            // let mut bpf = Ebpf::load_file("glrd.bpf.o")?;
            // listener::run(&mut bpf, engine, store, grpc).await?;

            println!("[OK] Gargoyle LRD arrancado — esperando eventos del kernel");
            Ok(())
        }
    }
}
