use std::collections::HashMap;

use crate::server::{self, GroupName, MemberData, member::Member};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Group {
    /// group name
    pub name: GroupName,  
    /// total members in the group
    pub(in crate::server) total_members: usize,
    /// Store all the members
    pub(in crate::server) members: MemberData,   
}

impl Group {
    /// If return is `true` then the group is created successfully
    fn create_group(group_name: &'static str, m_groups: &mut server::GroupData) -> bool {
        match m_groups.contains_key(group_name) {
            true => {
                tracing::warn!("group with `{}` already exist, cannot create another", group_name);
                return false;
            },
            false => {
                let group_data = Group {
                     name: group_name,
                     total_members: 0,
                     members: HashMap::new(),       
                };
                m_groups.insert(group_name, group_data);
                return true;                
            },
        }
    }
    
    /// If return is `true` then the member sucessfully join the group
    ///
    /// `group_name`: The group to join
    ///
    /// `m_member`: The member that will join the group `group_name`
    ///
    /// `m_groups`: All groups are stored here
    fn join_group(group_name: &str, m_member: &Member, m_groups: &mut server::GroupData) -> bool {
        if m_member.is_group_member {
            match m_member.group_name {
                Some(m_group_name) => {
                    if group_name == m_group_name {
                        tracing::debug!("`{}` is already part of the group `{}`", &m_member.username, &group_name);
                        return false;
                    } 
                    else {
                        tracing::debug!("`{}` can't join because he/she is part of another group `{}`", &m_member.username, &group_name);
                        return false;
                    }
                },
                // Don't need to handle for `None` type
                None => {}
            }
        }

        match m_groups.contains_key(group_name) {
            true => {
                let m_group = m_groups.get(group_name).unwrap();
                if m_group.total_members + 1 > server::MAX_GROUP_MEMBER {
                    tracing::error!("`{}` failed to join group `{}` due to max group members reached", m_member.username, group_name);
                    return false;
                }

                // Add the member to the group
                m_groups.get_mut(group_name)
                    .unwrap()
                    .members
                    .insert(m_member.username, m_member.clone())
                    .unwrap();

                tracing::debug!("`{}` joined group `{}`", m_member.username, group_name);
                return true;
            },
            false => {
                tracing::error!("group `{}` doesn't exist", group_name);
                return false;
            },
        }
    }
}
