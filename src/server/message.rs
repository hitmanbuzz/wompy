use crate::server::{self, GroupName, MAX_MSG_LEN, MemberName};

pub struct MemberMessage {
    member_name: MemberName,
    member_msg: Vec<String>,
}

pub struct Message {
    group_name: GroupName,
    group_msg: Vec<MemberMessage>,
}

impl Message {
    pub async fn send_msg(msg_data: &str, n_member: &super::Member, m_groups: &mut super::GroupData, m_members: &mut super::MemberData) -> bool {
        let member_name = n_member.username.as_str();
        if !server::Member::is_member_exist(member_name, m_members).await {
            tracing::error!("Member `{}` doesn't exist", member_name);
            return false;
        }
        
        if !n_member.is_group_member {
            tracing::error!("Member `{} is not a group member`", member_name);
            return false;
        }

        if !n_member.is_connected {
            tracing::error!("Member `{}` is offline", member_name);
            return false;
        }

        match server::Group::is_user_in_group(member_name, n_member.group_name.clone().unwrap().as_str(), &m_groups).await {
            true => {
                // Update on the member side
                m_members
                    .get_mut(member_name)
                    .unwrap()
                    .msg
                    .push(msg_data.to_string());

                // Update on the group side
                m_groups
                    .get_mut(n_member.group_name.clone().unwrap().as_str())
                    .unwrap()
                    .members
                    .get_mut(member_name)
                    .unwrap()
                    .msg
                    .push(msg_data.to_string());

                return true;
            },
            false => {
                tracing::error!("Member `{}` is not in group `{}`", member_name, &n_member.group_name.clone().unwrap());
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
