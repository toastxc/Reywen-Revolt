pub mod media {
    pub mod attachment;
    pub mod embeds;
    pub mod emoji;
}

pub mod channels {
    pub mod channel;
    pub mod channel_invite;
    pub mod channel_unread;
    pub mod message;
}

pub mod users {
    pub mod bot;
    pub mod user;
}

pub mod server {
    pub mod server;
    pub mod server_ban;
    pub mod server_member;
}

pub mod permissions {
    pub mod calculator;
    pub mod definitions;
    pub mod newcalc;
}
