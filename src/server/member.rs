use crate::server::{GroupName, MemberName};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Member {
    /// member username (String)
    pub(in crate::server) username: MemberName,
    /// member ip
    pub(in crate::server) ip: std::net::IpAddr,
    /// store all the msg for the member
    pub(in crate::server) msg: Vec<String>,
    /// is member connected to the server (online/offline) 
    pub(in crate::server) is_connected: bool,
    /// is the member part of a group
    pub(in crate::server) is_group_member: bool,
    /// use if `is_group_member` = true
    pub(in crate::server) group_name: Option<GroupName>,
}

impl Member {
    pub async fn create_member(member_name: &str, member_ip: std::net::IpAddr, m_members: &mut super::MemberData) -> bool {
        if Member::is_member_exist(member_name, m_members).await {
            return false;
        }

        let m = Member {
            username: member_name.to_string(),
            ip: member_ip,
            msg: Vec::new(),
            is_connected: true,
            is_group_member: false,
            group_name: None,
        };

        m_members.insert(member_name.to_string(), m).unwrap();
        return true;
    } 

    /// `true` = member exist in the server data
    pub async fn is_member_exist(member_name: &str, m_members: &super::MemberData) -> bool {
        match m_members.contains_key(member_name) {
            true => return true,
            false => return false,
        }
    }
}
