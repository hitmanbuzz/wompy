use std::collections::HashMap;
use crate::server::{group::Group, user::User};

pub mod tcp_server;
pub mod task;
pub mod message;
pub mod group;
pub mod user;

/// The server local IP Address
pub const IP_ADDR: &str = "127.0.0.1:4096";
/// The max characters allowed for a message
pub(in crate::server) const MAX_MSG_LEN: usize = 255;
/// The max members allow in a group
pub(in crate::server) const MAX_GROUP_USER: usize = 50;

pub(in crate::server) type UserName = String;
pub(in crate::server) type GroupName = String;

pub(in crate::server) type GroupData = HashMap<GroupName, Group>;
pub(in crate::server) type UserData = HashMap<UserName, User>;
