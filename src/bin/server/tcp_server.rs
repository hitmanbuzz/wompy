use std::{collections::HashMap, process::exit};
use crate::{group::Group, task::create_task, utils::{GroupData, UserData}};

#[allow(dead_code)]
#[derive(Debug)]
pub struct TcpServer {
    pub server_id: usize,
    /// The Local IP Address where the server is running
    server_ip: String,
    tcp_listener: tokio::net::TcpListener,
    /// Store all users
    ///
    /// Key -> UserName (String)
    users: UserData,
    /// Store all groups
    ///
    /// Key -> GroupName (String)
    groups: GroupData,
}

#[allow(dead_code)]
impl TcpServer {
    /// `ip_addr`: The IP Address where the server will run
    pub async fn create_server(ip_addr: &str) -> Self {
        let tcp_listener = tokio::net::TcpListener::bind(ip_addr).await.unwrap_or_else(|e| {
            match e.kind() {
                tokio::io::ErrorKind::AddrInUse => {
                    tracing::error!("ip: {} already used", ip_addr);
                    exit(69);
                },
                _ => {
                    tracing::error!("failed to bind ip: {}", ip_addr);
                    exit(67);
                },
            }
        });

        Self {
            server_id: 0,
            server_ip: ip_addr.to_string(),
            tcp_listener,
            users: HashMap::new(),
            groups: HashMap::new(),
        }
    }

    /// Use this after `create_server` function has been created
    pub async fn run_server(&self) -> anyhow::Result<()> {
        tracing::debug!(
            "[SERVER] IP: {} | ID: {}",
            self.server_ip, self.server_id
        );
        loop {
            let (tcp_stream, socket_addr) = self.tcp_listener.accept().await?;
            tracing::info!("new connection: {}", socket_addr);
            create_task(tcp_stream, socket_addr);
        }
    }

    pub async fn create_user(&mut self, user_name: &str) {
        todo!()
    }

    pub async fn create_group(&mut self, group_name: &str) {
        match Group::create_group(group_name, &mut self.groups).await {
            true => tracing::debug!("group with name `{}` has been created", group_name),
            false => tracing::warn!("group with `{}` already exist, cannot create another", group_name),
        }
    }

    pub async fn is_group_exist(&self, group_name: &str) -> bool {
        return Group::is_group_exist(group_name, &self.groups).await;
    }

    pub async fn get_total_users(&self) -> usize {
        self.users.len()
    }

    pub async fn get_total_groups(&self) -> usize {
        self.groups.len()
    }
}

