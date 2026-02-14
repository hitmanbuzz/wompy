pub mod tcp_server;
pub mod task;
pub mod message;

pub const IP_ADDR: &str = "127.0.0.1:4096";
pub const MAX_MSG_LEN: usize = 255;
pub const MAX_SERVER_MEMBER: usize = 50;
