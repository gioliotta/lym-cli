use clap::Parser;
use lym_cli::{run, ClapArgs};

#[tokio::main]
async fn main() {
    let arguments: ClapArgs = ClapArgs::parse();
    run(arguments).await
}

// Actualizar build en local (Ubuntu).
// sudo cp [ruta-del-exe] /usr/local/bin
// sudo cp /home/gixi/toding/programasao/lym_cli/target/release/lym /usr/local/bin
