use std::{collections::HashMap, process::exit};
use crate::server::{IP_ADDR, task::create_task};

#[allow(dead_code)]
#[derive(Debug)]
pub struct TcpServer {
    pub server_id: usize,
    tcp_listener: tokio::net::TcpListener,
    /// Store all members
    ///
    /// Key -> (MemberId, MemberName)
    members: super::MemberData,
    /// Store all groups
    ///
    /// Key -> (GroupId, GroupName)
    groups: super::GroupData,
}

#[allow(dead_code)]
impl TcpServer {
    /// `ip_addr`: The IP Address where the server will run
    pub async fn create_server(ip_addr: &str) -> Self {
        let tcp_listener = tokio::net::TcpListener::bind(ip_addr).await.unwrap_or_else(|e| {
            match e.kind() {
                tokio::io::ErrorKind::AddrInUse => {
                    tracing::error!("ip: {} already used", IP_ADDR);
                    exit(69);
                },
                _ => {
                    tracing::error!("failed to bind ip: {}", IP_ADDR);
                    exit(67);
                },
            }
        });

        Self {
            server_id: 0,
            tcp_listener,
            members: HashMap::new(),
            groups: HashMap::new(),
        }
    }

    /// Use this after `create_server` function has been created
    pub async fn run_server(&self) -> anyhow::Result<()> {
        tracing::debug!(
            "[SERVER] IP: {} | ID: {}",
            IP_ADDR, self.server_id
        );
        loop {
            let (tcp_stream, socket_addr) = self.tcp_listener.accept().await?;
            tracing::info!("new connection: {}", socket_addr);
            create_task(tcp_stream, socket_addr);
        }
    }

    pub async fn create_member(&mut self) {
        
    }

    pub async fn create_group(&mut self) {
        
    }

    pub async fn is_group_exist(&self, group_name: &str) {
        
    }

    pub async fn get_total_members(&self) -> usize {
        self.members.len()
    }

    pub async fn get_total_groups(&self) -> usize {
        self.groups.len()
    }
}

