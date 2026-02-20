use crate::group::Group;
use crate::user::User;
use crate::utils::{GroupData, GroupName, UserData, UserName};

#[allow(dead_code)]
pub struct UserMessage {
    user_name: UserName,
    user_msg: Vec<String>,
}

#[allow(dead_code)]
pub struct Message {
    group_name: GroupName,
    group_msg: Vec<UserMessage>,
}

#[allow(dead_code)]
impl Message {
    pub async fn send_msg(
        msg_data: &str,
        n_user: &User,
        m_groups: &mut GroupData,
        m_users: &mut UserData,
    ) -> bool {
        let user_name = n_user.username.as_str();
        if !User::is_user_exist(user_name, m_users).await {
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

        match Group::is_user_in_group(
            user_name,
            n_user.group_name.clone().unwrap().as_str(),
            &m_groups,
        )
        .await
        {
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
            }
            false => {
                tracing::error!(
                    "User `{}` is not in group `{}`",
                    user_name,
                    &n_user.group_name.clone().unwrap()
                );
                return false;
            }
        }
    }
}
