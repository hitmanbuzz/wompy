use crate::utils::{GroupName, UserName, UserData};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct User {
    /// user username (String)
    pub username: UserName,
    /// user ip
    pub ip: std::net::SocketAddr,
    /// store all the msg for the user
    pub msg: Vec<String>,
    /// is user connected to the server (online/offline) 
    pub is_connected: bool,
    /// is the user part of a group
    pub is_group_user: bool,
    /// use if `is_group_user` = true
    pub group_name: Option<GroupName>,
}

#[allow(dead_code)]
impl User {
    /// `true` = user exist in the server data
    pub async fn is_user_exist(user_name: &str, m_users: &UserData) -> bool {
        match m_users.contains_key(user_name) {
            true => return true,
            false => return false,
        }
    }
}

/// Handle the incoming user data from the client
pub async fn handle_user(user: &User, curr_msg: &str) {
    if user.msg.is_empty() {
        return;
    }
    
    tracing::info!("user: {} | group: {} | msg: {}", user.username, user.group_name.as_ref().unwrap(), curr_msg);
}
