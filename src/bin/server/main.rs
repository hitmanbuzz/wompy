use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::{tcp_server::TcpServer, utils::IP_ADDR};

mod group;
mod message;
mod task;
mod tcp_server;
mod user;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    eprint!("\x1B[2J\x1B[1;1H");

    let mut server = TcpServer::create_server(IP_ADDR).await;
    match server.run_server().await {
        Ok(_) => {}
        Err(e) => tracing::error!("failed to run server: {}", e),
    }

    Ok(())
}
