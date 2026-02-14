use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use crate::server::message::handle_msg_data;

pub fn create_task(tcp_stream: tokio::net::TcpStream, socket_addr: std::net::SocketAddr) {
    tokio::spawn(async move {
        let (reader, mut writer) = tcp_stream.into_split();
        let mut line = String::new();
        let mut reader = tokio::io::BufReader::new(reader);

        if let Err(e) = writer.write_all(format!("From Server: {}", line).as_bytes()).await {
            tracing::error!("failed to write to socket: {}", e);
        }

        loop {
            line.clear();

            match reader.read_line(&mut line).await {
                Ok(0) => {
                    tracing::info!("client disconnected: {}", socket_addr);
                    break;
                },
                Ok(_) => {
                    handle_msg_data(&socket_addr, &line.trim());
                },
                Err(e) => {
                    tracing::error!("error reading from socket: {}", e);
                    break;
                }
            }
        }
    });
}
