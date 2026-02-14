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
