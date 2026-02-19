use tokio::io::AsyncWriteExt;

pub async fn connect_server(ip_addr: &str) -> tokio::net::TcpStream {
    let new_stream = tokio::net::TcpStream::connect(ip_addr).await;

    match new_stream {
        Ok(stream) => return stream,
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::HostUnreachable => {
                    panic!("Host Unreachable (Not Found)");
                },
                std::io::ErrorKind::AddrNotAvailable => {
                    panic!("Host not found");
                },
                std::io::ErrorKind::ConnectionRefused => {
                    panic!("Got rejected by the host");
                }
                _ => {
                    panic!("Got Error: {}", e);
                }
            }
        },
    }
}

/// Send username and groupname after connecting to the server
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
