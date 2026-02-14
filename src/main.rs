use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
    
    let server = server::tcp_server::TcpServer::create_server(server::IP_ADDR).await;
    match server.run_server().await {
         Ok(_) => {},
         Err(e) => tracing::error!("failed to run server: {}", e),       
    }
    
    Ok(())
}

