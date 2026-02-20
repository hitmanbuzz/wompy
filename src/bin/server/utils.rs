use crate::{group::Group, user::User};
use std::collections::HashMap;

/// The server local IP Address
pub const IP_ADDR: &str = "127.0.0.1:4096";
/// The max members allow in a group
pub const MAX_GROUP_USER: usize = 50;

pub type UserName = String;
pub type GroupName = String;

pub type GroupData = HashMap<GroupName, Group>;
pub type UserData = HashMap<UserName, User>;
