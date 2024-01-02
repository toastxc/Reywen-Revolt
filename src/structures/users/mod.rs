use crate::impl_to_vec;
use crate::structures::media::attachment::File;
use serde::{Deserialize, Serialize};
pub mod bot;
fn if_false(t: &bool) -> bool {
    !t
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum RelationshipStatus {
    None,
    User,
    Friend,
    Outgoing,
    Incoming,
    Blocked,
    BlockedOther,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Relationship {
    #[serde(rename = "_id")]
    pub id: String,
    pub status: RelationshipStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Presence {
    /// User is online
    Online,
    /// User is not currently available
    Idle,
    /// User is focusing / will only receive mentions
    Focus,
    /// User is busy / will not receive any notifications
    Busy,
    /// User appears to be offline
    Invisible,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UserStatus {
    /// Custom status text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Current presence option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<Presence>,
}

impl UserStatus {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_text(&mut self, text: &str) -> Self {
        self.text = Some(String::from(text));
        self.to_owned()
    }
    pub fn set_presence(&mut self, presence: Presence) -> Self {
        self.presence = Some(presence);
        self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UserProfile {
    /// Text content on user's profile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Banner on user's profile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<File>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(i32)]
pub enum Badges {
    /// Revolt Developer
    Developer = 1,
    /// Helped translate Revolt
    Translator = 2,
    /// Monetarily supported Revolt
    Supporter = 4,
    /// Responsibly disclosed a security issue
    ResponsibleDisclosure = 8,
    /// Revolt Founder
    Founder = 16,
    /// Platform moderator
    PlatformModeration = 32,
    /// Active monetary supporter
    ActiveSupporter = 64,
    /// 🦊🦝
    Paw = 128,
    /// Joined as one of the first 1000 users in 2021
    EarlyAdopter = 256,
    /// Amogus
    ReservedRelevantJokeBadge1 = 512,
    /// Low resolution troll face
    ReservedRelevantJokeBadge2 = 1024,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(i32)]
pub enum Flags {
    /// User has been suspended from the platform
    Suspended = 1,
    /// User has deleted their account
    Deleted = 2,
    /// User was banned off the platform
    Banned = 4,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BotInformation {
    /// Id of the owner of this bot
    pub owner: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct User {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: String,
    /// Username
    pub username: String,
    /// User discriminator (#1234)
    pub discriminator: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// User's display name
    pub display_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Avatar attachment
    pub avatar: Option<File>,

    /// Relationships with other users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relations: Option<Vec<Relationship>>,

    /// Bitfield of user badges
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badges: Option<i32>,
    /// User's current status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    /// User's profile page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<UserProfile>,

    /// Enum of user flags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,
    /// Whether this user is privileged
    #[serde(skip_serializing_if = "if_false", default)]
    pub privileged: bool,
    /// Bot information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<BotInformation>,

    // ? Entries below should never be pushed to the database
    /// Current session user's relationship with this user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<RelationshipStatus>,
    /// Whether this user is currently online
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum FieldsUser {
    Avatar,
    StatusText,
    StatusPresence,
    ProfileContent,
    ProfileBackground,
}

pub enum UserHint {
    /// Could be either a user or a bot
    Any,
    /// Only match bots
    Bot,
    /// Only match users
    User,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataSendFriendRequest {
    pub username: String,
}
impl_to_vec!(DataSendFriendRequest);
impl DataSendFriendRequest {
    pub fn set_username(username: impl Into<String>) -> Self {
        Self {
            username: username.into(),
        }
    }
}

//https://api.revolt.chat/users/{target}
/// # User Data
#[derive(Serialize, Debug, Clone, Default)]
pub struct DataEditUser {
    /// Attachment Id for avatar
    pub avatar: Option<String>,
    /// New user status
    pub status: Option<UserStatus>,
    /// New user profile data
    ///
    /// This is applied as a partial.
    pub profile: Option<UserProfileData>,
    /// Bitfield of user badges
    pub badges: Option<i32>,
    /// Enum of user flags
    pub flags: Option<i32>,
    /// Fields to remove from user object
    pub remove: Option<Vec<FieldsUser>>,
}
impl_to_vec!(DataEditUser);
impl DataEditUser {
    pub fn set_avatar(&mut self, avatar: impl Into<String>) -> Self {
        self.avatar = Some(avatar.into());
        self.clone()
    }
    pub fn set_status(&mut self, status: impl Into<UserStatus>) -> Self {
        self.status = Some(status.into());
        self.clone()
    }
    pub fn set_profile(&mut self, profile: impl Into<UserProfileData>) -> Self {
        self.profile = Some(profile.into());
        self.clone()
    }
    pub fn set_badges(&mut self, badges: i32) -> Self {
        self.badges = Some(badges);
        self.clone()
    }
    pub fn set_flags(&mut self, flags: i32) -> Self {
        self.flags = Some(flags);
        self.clone()
    }
    pub fn set_remove(&mut self, remove: impl Into<Vec<FieldsUser>>) -> Self {
        self.remove = Some(remove.into());
        self.clone()
    }
    pub fn add_remove(&mut self, remove: impl Into<FieldsUser>) -> Self {
        match self.remove.clone() {
            Some(mut data) => {
                data.push(remove.into());
                self.remove = Some(data);
            }
            None => self.remove = Some(vec![remove.into()]),
        }
        self.clone()
    }

    pub fn new() -> Self {
        Default::default()
    }
}
#[derive(Serialize, Debug, Clone, Default)]
pub struct UserProfileData {
    /// Text to set as user profile description
    pub content: Option<String>,
    /// Attachment Id for background
    pub background: Option<String>,
}

impl UserProfileData {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn set_content(&mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self.clone()
    }
    pub fn set_background(&mut self, background: impl Into<String>) -> Self {
        self.background = Some(background.into());
        self.clone()
    }
}
/// # Mutual Friends and Servers Response
#[derive(Deserialize, Debug, Clone, Default)]
pub struct MutualResponse {
    /// Array of mutual user IDs that both users are friends with
    pub users: Vec<String>,
    /// Array of mutual server IDs that both users are in
    pub servers: Vec<String>,
}

/// # Flag Response
#[derive(Deserialize, Debug, Clone, Default)]
pub struct ResponseFlag {
    /// Flags
    pub flags: i32,
}
