use clap::Parser;
use lym_cli::{run, ClapArgs};

#[tokio::main]
async fn main() {
    let arguments: ClapArgs = ClapArgs::parse();
    run(arguments).await
}
