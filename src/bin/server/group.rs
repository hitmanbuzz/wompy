use crate::user::User;
use crate::utils::{GroupData, GroupName, MAX_GROUP_USER, UserData};
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Group {
    /// group name
    pub name: GroupName,
    /// total users in the group
    pub total_users: usize,
    /// Store all the users
    pub users: UserData,
}

#[allow(dead_code)]
impl Group {
    /// If return is `true` then the group is created successfully
    pub async fn create_group(group_name: &str, m_groups: &mut GroupData) -> bool {
        match m_groups.contains_key(group_name) {
            true => {
                return false;
            }
            false => {
                let group_data = Group {
                    name: group_name.to_string(),
                    total_users: 0,
                    users: HashMap::new(),
                };
                m_groups.insert(group_name.to_string(), group_data);
                return true;
            }
        }
    }

    /// If return is `true` then the user sucessfully join the group
    ///
    /// `group_name`: The group to join
    ///
    /// `m_user`: The user that will join the group `group_name`
    ///
    /// `m_groups`: All groups are stored here
    pub async fn join_group(m_user: &User, m_groups: &mut GroupData) -> bool {
        let group_name = m_user.group_name.as_ref().unwrap().as_str().trim();
        let username = m_user.username.as_str();

        // TODO: Implement user to leave a group
        // if m_user.is_group_user {
        //     if Group::is_user_in_group(&m_user.username, group_name.as_str(), m_groups).await {
        //         tracing::debug!("`{}` is already part of the group `{}`", &m_user.username, group_name.as_str());
        //         return false;
        //     }
        //     else {
        //         tracing::debug!("`{}` can't join because he/she is part of another group `{}`", &m_user.username, group_name.as_str());
        //         return false;
        //     }
        // }

        match Group::is_group_exist(group_name, m_groups).await {
            true => {
                let m_group = m_groups.get(group_name).unwrap();
                if m_group.total_users + 1 > MAX_GROUP_USER {
                    tracing::error!(
                        "`{}` failed to join group `{}` due to max group users reached",
                        username.trim(),
                        group_name.trim()
                    );
                    return false;
                }

                // Add the user to the group
                println!("adding this user to group: {:?}", m_user);
                m_groups
                    .get_mut(group_name)
                    .unwrap()
                    .users
                    .insert(username.to_string(), m_user.clone())
                    .unwrap();

                // increment users count after joining the group
                m_groups.get_mut(group_name).unwrap().total_users += 1;

                tracing::debug!("`{}` joined group `{}`", username.trim(), group_name.trim());
                return true;
            }
            false => {
                tracing::error!("group `{}` doesn't exist", group_name.trim());
                return false;
            }
        }
    }

    /// `true` = group exist
    pub async fn is_group_exist(group_name: &str, m_groups: &GroupData) -> bool {
        match m_groups.contains_key(group_name) {
            true => return true,
            false => return false,
        }
    }

    /// Check if a user is in a specific group
    pub async fn is_user_in_group(user_name: &str, group_name: &str, m_groups: &GroupData) -> bool {
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
