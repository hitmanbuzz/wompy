use crate::server::MAX_MSG_LEN;

pub struct Message {
    group_name: String,
}

/// Handle the incoming msg data from the clients
pub fn handle_msg_data(socket_addr: &std::net::SocketAddr, data_str: &str) {
    if data_str.is_empty() {
        return;
    }

    if data_str.len() > MAX_MSG_LEN {
        tracing::error!("message length is above max value: {}", data_str.len());
        return;
    }
    
    tracing::debug!("[{}] ≫ {}", socket_addr, data_str);
}
