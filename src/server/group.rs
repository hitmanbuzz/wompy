use std::collections::HashMap;
use crate::server::{self, GroupName, UserData, User};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Group {
    /// group name
    pub name: GroupName,  
    /// total users in the group
    pub(in crate::server) total_users: usize,
    /// Store all the users
    pub(in crate::server) users: UserData,   
}

impl Group {
    /// If return is `true` then the group is created successfully
    pub(in crate::server) async fn create_group(group_name: &str, m_groups: &mut server::GroupData) -> bool {
        match m_groups.contains_key(group_name) {
            true => {
                return false;
            },
            false => {
                let group_data = Group {
                     name: group_name.to_string(),
                     total_users: 0,
                     users: HashMap::new(),       
                };
                m_groups.insert(group_name.to_string(), group_data);
                return true;                
            },
        }
    }
    
    /// If return is `true` then the user sucessfully join the group
    ///
    /// `group_name`: The group to join
    ///
    /// `m_user`: The user that will join the group `group_name`
    ///
    /// `m_groups`: All groups are stored here
    pub(in crate::server) async fn join_group(group_name: &str, m_user: &User, m_groups: &mut server::GroupData) -> bool {
        if m_user.is_group_user {
            if let Some(ref m_group_name) = m_user.group_name {
                if group_name == m_group_name {
                    tracing::debug!("`{}` is already part of the group `{}`", &m_user.username, &group_name);
                    return false;
                } 
                else {
                    tracing::debug!("`{}` can't join because he/she is part of another group `{}`", &m_user.username, &group_name);
                    return false;
                }
            }
        }

        match Group::is_group_exist(group_name, m_groups).await {
            true => {
                let m_group = m_groups.get(group_name).unwrap();
                if m_group.total_users + 1 > server::MAX_GROUP_USER {
                    tracing::error!("`{}` failed to join group `{}` due to max group users reached", m_user.username, group_name);
                    return false;
                }

                // Add the user to the group
                m_groups.get_mut(group_name)
                    .unwrap()
                    .users
                    .insert(m_user.username.clone(), m_user.clone())
                    .unwrap();

                // increment users count after joining the group
                m_groups.get_mut(group_name)
                    .unwrap()
                    .total_users += 1;

                tracing::debug!("`{}` joined group `{}`", m_user.username, group_name);
                return true;
            },
            false => {
                tracing::error!("group `{}` doesn't exist", group_name);
                return false;
            },
        }
    }

    /// `true` = group exist
    pub(in crate::server) async fn is_group_exist(group_name: &str, m_groups: &server::GroupData) -> bool {
        match m_groups.contains_key(group_name) {
            true => return true,
            false => return false,
        }
    }

    /// Check if a user is in a specific group
    pub(in crate::server) async fn is_user_in_group(user_name: &str, group_name: &str, m_groups: &server::GroupData) -> bool {
        if Group::is_group_exist(group_name, m_groups).await {
            let g = m_groups.get(group_name).unwrap();
            match g.users.contains_key(user_name) {
                true => return true,
                false => return false,
            }
        }

        return false;
    }
}
