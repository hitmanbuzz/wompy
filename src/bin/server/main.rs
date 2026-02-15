use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::{tcp_server::TcpServer, utils::IP_ADDR};

mod tcp_server;
mod task;
mod message;
mod group;
mod user;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
    
    let server = TcpServer::create_server(IP_ADDR).await;
    match server.run_server().await {
         Ok(_) => {},
         Err(e) => tracing::error!("failed to run server: {}", e),       
    }
    
    Ok(())
}

