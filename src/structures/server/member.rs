use crate::structures::server::Role;
use crate::{
    client::methods::opt_vec_add,
    impl_to_vec,
    structures::{media::attachment::File, users::User},
};
use iso8601_timestamp::Timestamp;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Composite primary key consisting of server and user id
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MemberCompositeKey {
    /// Server Id
    pub server: String,
    /// User Id
    pub user: String,
}

/// Representation of a member of a server on Revolt
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Member {
    /// Unique member id
    #[serde(rename = "_id")]
    pub id: MemberCompositeKey,

    /// Time at which this user joined the server
    pub joined_at: Timestamp,

    /// Member's nickname
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// Avatar attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<File>,

    /// Member's roles
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub roles: Vec<String>,
    /// Timestamp this member is timed out until
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<Timestamp>,
}

/// Optional fields on server member object
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum FieldsMember {
    Nickname,
    Avatar,
    Roles,
    Timeout,
}

/// Member removal intention
pub enum RemovalIntention {
    Leave,
    Kick,
    Ban,
}

/// # Member List
///
/// Both lists are sorted by ID.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseMemberAll {
    /// List of members
    pub members: Vec<Member>,
    /// List of users
    pub users: Vec<User>,
}
/// # Member Data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataMemberEdit {
    /// Member nickname
    nickname: Option<String>,
    /// Attachment Id to set for avatar
    avatar: Option<String>,
    /// Array of role ids
    roles: Option<Vec<String>>,
    /// Timestamp this member is timed out until
    timeout: Option<Timestamp>,
    /// Fields to remove from channel object
    remove: Option<Vec<FieldsMember>>,
}
impl_to_vec!(DataMemberEdit);
impl DataMemberEdit {
    pub fn set_nickname(&mut self, nickname: impl Into<String>) -> Self {
        self.nickname = Some(nickname.into());
        self.clone()
    }
    pub fn set_avatar(&mut self, avatar: impl Into<String>) -> Self {
        self.avatar = Some(avatar.into());
        self.clone()
    }
    pub fn set_roles(&mut self, roles: impl Into<Vec<String>>) -> Self {
        self.roles = Some(roles.into());
        self.clone()
    }
    pub fn add_role(mut self, role: impl Into<String> + Clone) -> Self {
        opt_vec_add(&mut self.roles, role.into());
        self.clone()
    }
    pub fn set_timeout(&mut self, timeout: impl Into<Timestamp>) -> Self {
        self.timeout = Some(timeout.into());
        self.to_owned()
    }

    pub fn add_remove(&mut self, remove: impl Into<FieldsMember>) -> Self {
        opt_vec_add(&mut self.remove, remove.into());
        self.to_owned()
    }
    pub fn set_remove(&mut self, remove: impl Into<Vec<FieldsMember>>) -> Self {
        self.remove = Some(remove.into());
        self.to_owned()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberWithRoles {
    pub member: Member,
    pub roles: HashMap<String, Role>,
}
