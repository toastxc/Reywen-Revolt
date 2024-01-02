use crate::impl_to_vec;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DataCreateGroup {
    /// Group name (min: 1, max: 32)
    pub name: String,
    /// Group description (length min: 0, max: 1024)
    pub description: Option<String>,
    /// Array of user IDs to add to the group
    ///
    /// Must be friends with these users.
    /// Length min: 0, max: 49
    pub users: Vec<String>,
    /// Whether this group is age-restricted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
}
impl_to_vec!(DataCreateGroup);

impl DataCreateGroup {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            ..Default::default()
        }
    }
    pub fn set_description(&mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self.clone()
    }

    pub fn add_user(&mut self, user: impl Into<String>) -> Self {
        self.users.push(user.into());
        self.clone()
    }
    pub fn set_users(&mut self, users: impl Into<Vec<String>>) -> Self {
        self.users = users.into();
        self.clone()
    }

    pub fn set_nsfw(&mut self, nsfw: bool) -> Self {
        self.nsfw = Some(nsfw);
        self.clone()
    }
}
