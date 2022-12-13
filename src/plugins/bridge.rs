use crate::{Auth, RMessage, rev_user, rev_convert_reply, rev_send, lib::message::*};
use crate::fs_str;
use serde::{Serialize, Deserialize};
    
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BrConf {
    pub enabled: bool,
    pub channel_1: String,
    pub channel_2: String,
}

//pub fn conf_error(details_in: 
pub async fn br_main(auth: Auth, input_message: RMessage) {

    let conf = fs_str("config/bridge.json");

    match conf {
        Ok(_) => {},
        Err(e) => panic!("failed to read config/message.json\n{e}"),
    };

    let bridge: BrConf = serde_json::from_str(&conf.unwrap())
        .expect("Failed to deser message.json");


    if !bridge.enabled  {
        return
    };

    // removing feedback loop
    if input_message.author == auth.bot_id && input_message.masquerade.is_some() {
        return
    };


    let (chan1, chan2) = (bridge.channel_1, bridge.channel_2);


    // channel switch
    let mut chan_rec = String::new();
    if input_message.channel == chan1 {
       chan_rec = chan2;
    }else if input_message.channel == chan2 {
       chan_rec = chan1;
    };

    
    let mut message = input_message.clone();
    
    message.channel = chan_rec;

    let br_masq: Masquerade;

    // masq switch - if user has no masquerade: pull from user info API
    // else - port over masquerade details 
    if input_message.masquerade.is_none()  {

        // API get masq
        
        let user1 = rev_user(&auth.token, &input_message.author).await;
        
        let user = match user1 {
          
            Some(a) =>  a,
            None => {println!("REV_USER_ERR: failed to get details for {}", input_message.author); return},  
        };
        
        let pfplink:String = match user.avatar {
            None => "https://api.revolt.chat/users/01GKWVQP8JP1TEZ52AR1NZVM1J/default_avatar".to_string(),
            Some(r) =>  format!("https://autumn.revolt.chat/avatars/{}", r.id),
        };

        //let pfp = format!("https://autumn.revolt.chat/avatars/{}", pfplink);

        br_masq = Masquerade {
            name: Some(user.username),
            avatar: Some(pfplink),
            colour: None
        };
        
    }else {
        
        // translate masq
        br_masq = Masquerade {
            name: message.masquerade.as_ref().unwrap().name.clone(),
            avatar: message.masquerade.as_ref().unwrap().avatar.clone(),
            colour: None
        };  

    };

    // message for rev_send
    let payload = RMessagePayload {
        content: message.content,
        attachments: None,
        replies: rev_convert_reply(input_message.replies).await,
        masquerade: Some(br_masq),
    };

    rev_send(&auth.token, &message.channel, payload).await;

}

