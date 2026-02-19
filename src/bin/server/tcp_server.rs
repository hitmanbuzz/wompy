use std::{collections::HashMap, process::exit, sync::Arc};
use tokio::sync::Mutex;

use crate::{group::Group, task::create_task, utils::{GroupData, UserData}};

#[allow(dead_code)]
#[derive(Debug)]
pub struct TcpServer {
    /// The Local IP Address where the server is running
    server_ip: String,
    tcp_listener: tokio::net::TcpListener,
    /// Store all users
    ///
    /// Key -> UserName (String)
    pub users: Arc<Mutex<UserData>>,
    /// Store all groups
    ///
    /// Key -> GroupName (String)
    pub groups: Arc<Mutex<GroupData>>,
}

#[allow(dead_code, unused_variables)]
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
            server_ip: ip_addr.to_string(),
            tcp_listener,
            users: Arc::new(Mutex::new(HashMap::new())),
            groups: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Use this after `create_server` function has been created
    pub async fn run_server(&mut self) -> anyhow::Result<()> {
        tracing::debug!("[SERVER] IP: {}", self.server_ip);
        loop {
            let (tcp_stream, socket_addr) = self.tcp_listener.accept().await?;
            tracing::debug!("new user connected with ip: {}", socket_addr);
            create_task(
                tcp_stream,
                socket_addr,
                Arc::clone(&self.users),
                Arc::clone(&self.groups)
            );
        }
    }

    pub async fn create_group(&mut self, group_name: &str) {
        let mut groups = self.groups.lock().await;
        match Group::create_group(group_name, &mut groups).await {
            true => tracing::debug!("group with name `{}` has been created", group_name),
            false => tracing::warn!("group with `{}` already exist, cannot create another", group_name),
        }
    }

    pub async fn is_group_exist(&self, group_name: &str) -> bool {
        let groups = self.groups.lock().await;
        return Group::is_group_exist(group_name, &groups).await;
    }

    pub async fn get_total_users(&self) -> usize {
        let users = self.users.lock().await;
        return users.len();
    }

    pub async fn get_total_groups(&self) -> usize {
        let groups = self.groups.lock().await;
        return groups.len();
    }
}

