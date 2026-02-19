use std::sync::Arc;
use crate::user::handle_user;
use tokio::io::AsyncBufReadExt;
use tokio::sync::Mutex;

pub fn create_task(
    tcp_stream: tokio::net::TcpStream,
    socket_addr: std::net::SocketAddr,
    server_users: Arc<Mutex<crate::utils::UserData>>,
    server_groups: Arc<Mutex<crate::utils::GroupData>>,
) {
    tokio::spawn(async move {
        let (reader, _) = tcp_stream.into_split();
        let (mut username, mut good_user) = (String::new(), false);
        let (mut groupname, mut good_group) = (String::new(), false);
        let mut msg = String::new();
        let mut reader = tokio::io::BufReader::new(reader);
        username.clear();

        let mut users = server_users.lock().await;
        let mut groups = server_groups.lock().await;

        // necessarilly this check is not needeed since the client side also check it
        // but it will be kept for now
        while username.is_empty() || groupname.is_empty() {
            // Receive username from client
            if !good_user {
                match reader.read_line(&mut username).await {
                    Ok(0) => {
                        tracing::debug!("[{}] disconnected", username);
                        break;
                    }
                    Ok(_) => {
                        good_user = true;
                    }
                    Err(e) => {
                        tracing::error!("error reading from socket: {}", e);
                        break;
                    }
                }
            }

            // Receive group name from client
            if !good_group {
                match reader.read_line(&mut groupname).await {
                    Ok(0) => {
                        tracing::debug!("[{}] disconnected", username);
                        break;
                    }
                    Ok(_) => {
                        good_group = true;
                    }
                    Err(e) => {
                        tracing::error!("error reading from socket: {}", e);
                        break;
                    }
                }
            }
        }

        tracing::debug!("[{}] just joined!!!", username.trim());

        while !username.is_empty() && !groupname.is_empty() {
            msg.clear();

            match reader.read_line(&mut msg).await {
                Ok(0) => {
                    tracing::debug!("[{}] disconnected", username);
                    break;
                },
                Ok(_) => {
                    let mut new_user = crate::user::User {
                        username: username.clone(),
                        ip: socket_addr,
                        msg: Vec::new(),
                        is_connected: true,
                        is_group_user: true,
                        group_name: Some(groupname.clone()),
                    };

                    new_user.group_name = Some(groupname.clone());
                    new_user.msg.push(msg.trim().to_string());
                    users.insert(new_user.username.clone(), new_user.clone());
                    handle_user(&new_user, msg.trim()).await;
                    crate::group::Group::join_group(
                        &new_user,
                        &mut groups
                    ).await;
                },
                Err(e) => {
                    tracing::error!("error reading from socket: {}", e);
                    break;
                }
            }
        }
    });
}
