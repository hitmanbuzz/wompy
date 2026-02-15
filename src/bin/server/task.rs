use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use crate::message::handle_msg_data;

pub fn create_task(tcp_stream: tokio::net::TcpStream, socket_addr: std::net::SocketAddr) {
    tokio::spawn(async move {
        let (reader, mut writer) = tcp_stream.into_split();
        let mut msg = String::new();
        let mut reader = tokio::io::BufReader::new(reader);

        if let Err(e) = writer.write_all(format!("From Server: {}", msg).as_bytes()).await {
            tracing::error!("failed to write to socket: {}", e);
        }

        loop {
            msg.clear();

            match reader.read_line(&mut msg).await {
                Ok(0) => {
                    tracing::info!("client disconnected: {}", socket_addr);
                    break;
                },
                Ok(_) => {
                    handle_msg_data(&socket_addr, &msg.trim());
                },
                Err(e) => {
                    tracing::error!("error reading from socket: {}", e);
                    break;
                }
            }
        }
    });
}
