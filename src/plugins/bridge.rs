use crate::{MainConf, RMessage, rev_user, rev_convert_reply, rev_send, lib::message::*};

//pub fn conf_error(details_in: 
pub async fn br_main(details: MainConf, input_message: RMessage) {

    if details.bridge.bridge_enabled == false {
        return
    };

    let auth = details.auth.clone();
    let br = details.bridge.clone();

    // removing feedback loop
    if input_message.author == auth.bot_id && input_message.masquerade != None {
        return
    };


    let (chan1, chan2) = (br.channel_1, br.channel_2);


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
    if input_message.masquerade == None {

        // API get masq
        
        let user1 = rev_user(auth.clone(), input_message.author.clone()).await;

        let user = match user1 {
            Ok(_) => user1.expect("failed to GET user details"),
            Err(_)  => return
        };

        let pfplink = user.avatar.unwrap().id;

        let pfp = format!("https://autumn.revolt.chat/avatars/{pfplink}");

        br_masq = Masquerade {
            name: Some(user.username),
            avatar: Some(pfp),
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
        content: message.content.clone(),
        attachments: None,
        replies: rev_convert_reply(input_message.replies).await,
        masquerade: Some(br_masq),
    };

    rev_send(auth, message, payload).await;

}

