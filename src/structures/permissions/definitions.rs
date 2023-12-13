use crate::impl_to_vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(u64)]
pub enum Permission {
    // * Generic permissions
    /// Manage the channel or channels on the server
    ManageChannel = 1 << 0,
    /// Manage the server
    ManageServer = 1 << 1,
    /// Manage permissions on servers or channels
    ManagePermissions = 1 << 2,
    /// Manage roles on server
    ManageRole = 1 << 3,
    /// Manage server customisation (includes emoji)
    ManageCustomisation = 1 << 4,

    // % 1 bit reserved

    // * Member permissions
    /// Kick other members below their ranking
    KickMembers = 1 << 6,
    /// Ban other members below their ranking
    BanMembers = 1 << 7,
    /// Timeout other members below their ranking
    TimeoutMembers = 1 << 8,
    /// Assign roles to members below their ranking
    AssignRoles = 1 << 9,
    /// Change own nickname
    ChangeNickname = 1 << 10,
    /// Change or remove other's nicknames below their ranking
    ManageNicknames = 1 << 11,
    /// Change own avatar
    ChangeAvatar = 1 << 12,
    /// Remove other's avatars below their ranking
    RemoveAvatars = 1 << 13,

    // % 7 bits reserved

    // * Channel permissions
    /// View a channel
    ViewChannel = 1 << 20,
    /// Read a channel's past message history
    ReadMessageHistory = 1 << 21,
    /// Send a message in a channel
    SendMessage = 1 << 22,
    /// Delete messages in a channel
    ManageMessages = 1 << 23,
    /// Manage webhook entries on a channel
    ManageWebhooks = 1 << 24,
    /// Create invites to this channel
    InviteOthers = 1 << 25,
    /// Send embedded content in this channel
    SendEmbeds = 1 << 26,
    /// Send attachments and media in this channel
    UploadFiles = 1 << 27,
    /// Masquerade messages using custom nickname and avatar
    Masquerade = 1 << 28,
    /// React to messages with emojis
    React = 1 << 29,

    // * Voice permissions
    /// Connect to a voice channel
    Connect = 1 << 30,
    /// Speak in a voice call
    Speak = 1 << 31,
    /// Share video in a voice call
    Video = 1 << 32,
    /// Mute other members with lower ranking in a voice call
    MuteMembers = 1 << 33,
    /// Deafen other members with lower ranking in a voice call
    DeafenMembers = 1 << 34,
    /// Move members between voice channels
    MoveMembers = 1 << 35,

    // * Misc. permissions
    // % Bits 36 to 52: free area
    // % Bits 53 to 64: do not use

    // * Grant all permissions
    /// Safely grant all permissions
    GrantAllSafe = 0x000F_FFFF_FFFF_FFFF,

    /// Grant all permissions
    GrantAll = u64::MAX,
}

/// Representation of a single permission override
#[derive(Deserialize, Serialize, Debug, Clone, Copy, Default)]
pub struct Override {
    /// Allow bit flags
    pub allow: u64,
    /// Disallow bit flags
    pub deny: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, Default)]
pub struct OverrideField {
    /// Allow bit flags
    pub allow: i64,
    /// Disallow bit flags
    pub deny: i64,
}

impl From<OverrideField> for Override {
    fn from(value: OverrideField) -> Self {
        Self {
            allow: value.allow as u64,
            deny: value.deny as u64,
        }
    }
}

impl Override {
    pub fn new() -> Self {
        Default::default()
    }
}

/// # Permission Value
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Field {
    /// Allow / deny values to set for this role
    pub permissions: Override,
}
impl_to_vec!(Field);

/// Permission values to set for members in a `Group`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Value {
    /// Allow / deny values to set for this role
    pub permissions: u64,
}
impl_to_vec!(Value);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PermissionData {
    pub value: Value,
    pub field: Field,
}
impl_to_vec!(PermissionData);
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Permissions {
    pub allow: Vec<Permission>,
    pub deny: Vec<Permission>,
}
