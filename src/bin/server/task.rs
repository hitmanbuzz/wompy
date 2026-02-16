use crate::message::handle_msg_data;
use tokio::io::AsyncBufReadExt;

pub fn create_task(tcp_stream: tokio::net::TcpStream, socket_addr: std::net::SocketAddr) {
    tokio::spawn(async move {
        let (reader, _) = tcp_stream.into_split();
        let mut username = String::new();
        let mut group_name = String::new();
        let mut msg = String::new();
        let mut reader = tokio::io::BufReader::new(reader);

        // Send the initial needed message back to client before chatting start
        while username.len() == 0 || group_name.len() == 0 {
            // Receive username from client
            match reader.read_line(&mut username).await {
                Ok(0) => {
                    tracing::debug!("client disconnected: {}", socket_addr);
                }
                Ok(_) => {}
                Err(e) => {
                    tracing::error!("error reading from socket: {}", e);
                    break;
                }
            }

            // Receive group name from client
            match reader.read_line(&mut group_name).await {
                Ok(0) => {
                    tracing::debug!("client disconnected: {}", socket_addr);
                }
                Ok(_) => {}
                Err(e) => {
                    tracing::error!("error reading from socket: {}", e);
                    break;
                }
            }
        }

        // Only allow receiving from client if the client has username and group name
        while username.len() > 0 && group_name.len() > 0 {
            msg.clear();

            match reader.read_line(&mut msg).await {
                Ok(0) => {
                    tracing::debug!("client disconnected: {}", socket_addr);
                    break;
                },
                Ok(_) => {
                    handle_msg_data(
                        &socket_addr,
                        &msg.trim(),
                        &username.trim(),
                        &group_name.trim(),
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
