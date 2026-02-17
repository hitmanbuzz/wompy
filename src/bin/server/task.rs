use crate::message::handle_msg_data;
use tokio::io::AsyncBufReadExt;

pub fn create_task(tcp_stream: tokio::net::TcpStream, socket_addr: std::net::SocketAddr) {
    tokio::spawn(async move {
        let (reader, _) = tcp_stream.into_split();
        let mut username = String::new();
        let mut msg = String::new();
        let mut reader = tokio::io::BufReader::new(reader);
        username.clear();

        while username.len() == 0 {
            // Receive username from client
            match reader.read_line(&mut username).await {
                Ok(0) => {
                    tracing::debug!("[{}] disconnected", username);
                    break;
                }
                Ok(_) => {}
                Err(e) => {
                    tracing::error!("error reading from socket: {}", e);
                    break;
                }
            }
        }

        tracing::debug!("[{}] just joined!!!", username.trim());

        while username.len() > 0 {
            msg.clear();

            match reader.read_line(&mut msg).await {
                Ok(0) => {
                    tracing::debug!("[{}] disconnected", username);
                    break;
                },
                Ok(_) => {
                    handle_msg_data(
                        &socket_addr,
                        &msg.trim(),
                        &username.trim(),
                    );
                },
                Err(e) => {
                    tracing::error!("error reading from socket: {}", e);
                    break;
                }
            }
        }
    });
}
