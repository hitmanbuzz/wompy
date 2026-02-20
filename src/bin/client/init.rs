/*
This rust file contains the code for the client to send and receive data from the server.
The gui part is in `client/main.rs`
*/

use tokio::io::AsyncWriteExt;

pub async fn connect_server(ip_addr: &str) -> tokio::net::TcpStream {
    let new_stream = tokio::net::TcpStream::connect(ip_addr).await;

    match new_stream {
        Ok(stream) => return stream,
        Err(e) => match e.kind() {
            std::io::ErrorKind::HostUnreachable => {
                panic!("Host Unreachable (Not Found)");
            }
            std::io::ErrorKind::AddrNotAvailable => {
                panic!("Host not found");
            }
            std::io::ErrorKind::ConnectionRefused => {
                panic!("Got rejected by the host");
            }
            _ => {
                panic!("Got Error: {}", e);
            }
        },
    }
}

/// Send username and groupname after connecting to the server
///
/// This will be done after the connection is made with the server
pub async fn send_init_data(
    user_name: &str,
    group_name: &str,
    stream: &mut tokio::net::TcpStream,
) -> bool {
    let username = format!("{}\n", user_name);
    let groupname = format!("{}\n", group_name);

    let (mut user_good, mut group_good) = (false, false);

    if let Ok(_) = stream.write_all(username.as_bytes()).await {
        user_good = true;
    }

    if let Ok(_) = stream.write_all(groupname.as_bytes()).await {
        group_good = true;
    }

    if user_good && group_good {
        tracing::info!(
            "user `{}` has joined the server in group: `{}`",
            username.trim(),
            groupname.trim()
        );
        return true;
    }

    return false;
}

/// Send chat msg to the server's group
pub async fn send_chat_msg(
    user_msg: &str,
    user_all_msg: &mut Vec<String>,
    stream: &mut tokio::net::TcpStream,
) {
    let mut is_msg_send = false;

    if let Ok(_) = stream.write_all(user_msg.as_bytes()).await {
        is_msg_send = true;
        user_all_msg.push(user_msg.to_string());
    }

    if is_msg_send {
        tracing::debug!("message send: `{}`", user_msg);
    } else {
        tracing::error!("failed to send message");
    }
}

/// Receive chat msg from the other users from the servers' group
pub async fn receive_chat_msg(stream: &mut tokio::net::TcpStream) {
    todo!()
}
