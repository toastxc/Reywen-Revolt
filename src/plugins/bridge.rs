// external
use serde::{Deserialize, Serialize};
// internal
use crate::{
    lib::{fs::fs_to_str, lreywen::crash_condition, oop::Reywen, rev_x::rev_convert_reply},
    structs::{
        auth::Auth,
        message::{Masquerade, RMessage, RMessagePayload},
    },
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BrConf {
    pub enabled: bool,
    pub channel_1: String,
    pub channel_2: String,
}

pub async fn br_main(auth: Auth, input_message: &RMessage) {
    // import config
    let conf_str =
        fs_to_str("config/bridge.json").expect("failed to read config/message.json\n{e}");

    let conf: BrConf = serde_json::from_str(&conf_str).expect("Failed to deser message.json");

    // fail conditions
    if !conf.enabled {
        return;
    };

    crash_condition(input_message, None);

    if auth.bot_id.is_empty() {
        println!("WARN: bot ID is empty, this can lead to undefined behavior");
    };

    // removing feedback loop
    if input_message.author == auth.bot_id && input_message.masquerade.is_some() {
        return;
    };

    // channel switcher
    // i want a better solution but cant think of one
    let mut chan_rec = String::new();
    if input_message.channel == conf.channel_1 {
        chan_rec = conf.channel_2;
    } else if input_message.channel == conf.channel_2 {
        chan_rec = conf.channel_1;
    };

    // made input mutable for the input channel to be changed
    let mut message = input_message.clone();
    message.channel = chan_rec;

    // due to how weird this plugin is by nature, the client needs to be created later
    let client = Reywen::new(auth.clone(), &message);

    let mut br_masq: Masquerade = Masquerade::new();

    //if user has no masquerade: pull from user info API
    if input_message.masquerade.is_none() {
        // moved to external function (its awful)
        br_masq = masq_from_user(&input_message.author, client.clone()).await;

        // else - port over masquerade details from input message
    } else {
        let in_masq = message.masquerade.unwrap();

        // translates masq values if applicable
        if in_masq.name.is_some() {
            br_masq = br_masq.name(&in_masq.name.unwrap());
        };
        if in_masq.avatar.is_some() {
            br_masq = br_masq.name(&in_masq.avatar.unwrap());
        };
        if in_masq.colour.is_some() {
            br_masq = br_masq.name(&in_masq.colour.unwrap());
        };
    };

    // construct payload and send
    let mut payload = RMessagePayload::new()
        .content(&input_message.clone().content.unwrap())
        .masquerade(br_masq);

    // weird custom method - converts replies from websocket to API
    let replies = rev_convert_reply(input_message.replies.clone());

    if replies.is_some() {
        payload = payload.replies(replies.unwrap());
    };
    client.send(payload).await;
}

async fn masq_from_user(author: &str, client: Reywen) -> Masquerade {
    let user = client.get_user(author).await;

    if user.is_some() {
        let user = user.unwrap();

        let avatar = match user.avatar {
            None => None,
            Some(r) => Some(format!("https://autumn.revolt.chat/avatars/{}", r.id)),
        };

        let mut masq = Masquerade::new().name(&user.username);

        if avatar.is_some() {
            masq = masq.avatar(&avatar.unwrap());
        };
        return masq;
    };

    Masquerade::new()
}
