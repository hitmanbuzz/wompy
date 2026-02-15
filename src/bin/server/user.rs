use crate::utils::{GroupName, UserName, UserData};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct User {
    /// user username (String)
    pub username: UserName,
    /// user ip
    pub ip: std::net::IpAddr,
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
    pub async fn create_user(user_name: &str, user_ip: std::net::IpAddr, m_users: &mut UserData) -> bool {
        if User::is_user_exist(user_name, m_users).await {
            return false;
        }

        let m = User {
            username: user_name.to_string(),
            ip: user_ip,
            msg: Vec::new(),
            is_connected: true,
            is_group_user: false,
            group_name: None,
        };

        m_users.insert(user_name.to_string(), m).unwrap();
        return true;
    } 

    /// `true` = user exist in the server data
    pub async fn is_user_exist(user_name: &str, m_users: &UserData) -> bool {
        match m_users.contains_key(user_name) {
            true => return true,
            false => return false,
        }
    }
}
