use crate::server::{self, GroupName, MAX_MSG_LEN, UserName};

pub struct UserMessage {
    user_name: UserName,
    user_msg: Vec<String>,
}

pub struct Message {
    group_name: GroupName,
    group_msg: Vec<UserMessage>,
}

impl Message {
    pub async fn send_msg(msg_data: &str, n_user: &super::User, m_groups: &mut super::GroupData, m_users: &mut super::UserData) -> bool {
        let user_name = n_user.username.as_str();
        if !server::User::is_user_exist(user_name, m_users).await {
            tracing::error!("User `{}` doesn't exist", user_name);
            return false;
        }
        
        if !n_user.is_group_user {
            tracing::error!("User `{} is not a group user`", user_name);
            return false;
        }

        if !n_user.is_connected {
            tracing::error!("User `{}` is offline", user_name);
            return false;
        }

        match server::Group::is_user_in_group(user_name, n_user.group_name.clone().unwrap().as_str(), &m_groups).await {
            true => {
                // Update on the user side
                m_users
                    .get_mut(user_name)
                    .unwrap()
                    .msg
                    .push(msg_data.to_string());

                // Update on the group side
                m_groups
                    .get_mut(n_user.group_name.clone().unwrap().as_str())
                    .unwrap()
                    .users
                    .get_mut(user_name)
                    .unwrap()
                    .msg
                    .push(msg_data.to_string());

                return true;
            },
            false => {
                tracing::error!("User `{}` is not in group `{}`", user_name, &n_user.group_name.clone().unwrap());
                return false;
            },
        }
    }
}

/// Handle the incoming msg data from the clients
pub fn handle_msg_data(socket_addr: &std::net::SocketAddr, msg_data: &str) {
    if msg_data.is_empty() {
        return;
    }

    if msg_data.len() > MAX_MSG_LEN {
        tracing::error!("message length is above max value: {}", msg_data.len());
        return;
    }
    
    tracing::debug!("[{}] ≫ {}", socket_addr, msg_data);
}
