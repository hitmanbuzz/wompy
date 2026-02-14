use std::process::exit;
use crate::server::{IP_ADDR, MAX_SERVER_MEMBER, task::create_task};

#[allow(dead_code)]
#[derive(Debug)]
pub struct ServerMember {
    member_id: usize,
    // `member_username` will not be use for now
    member_username: String,
    member_ip: std::net::IpAddr,
    /// Store all the msg from the same member
    member_msg: Vec<String>,
    is_member_connected: bool,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct TcpServer {
    // `id` will not be use for now
    server_id: usize,
    tcp_listener: tokio::net::TcpListener,
    /// The server allow only limited members
    ///
    /// `MAX_SERVER_MEMBER` = Total Numbers of Members allowed
    server_members: Option<[ServerMember; MAX_SERVER_MEMBER]>,
}

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
            server_members: None,
        }
    }

    /// Use this after `create_server` function has been created
    pub async fn run_server(&self) -> anyhow::Result<()> {
        tracing::debug!("server running at: {}", IP_ADDR);
        loop {
            let (tcp_stream, socket_addr) = self.tcp_listener.accept().await?;
            tracing::info!("new connection: {}", socket_addr);
            create_task(tcp_stream, socket_addr);
        }
    }
}

