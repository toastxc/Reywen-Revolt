// an abstraction layer for RevX2

use crate::lib::message::{RMessage, RReplies, RMessagePayload, Masquerade};
use crate::rev_x::*;

pub fn reyshell_masq(content: &str) -> RMessagePayload {

      let masq = Masquerade {
        name: Some(String::from("ReyShell")),
        avatar: Some(String::from("https://toastxc.xyz/TXCS/reyshell.png")),
        colour: None,
      };

    RMessagePayload {
        content: Some(String::from(content)),
        attachments: None,
        replies: None,
        masquerade:  Some(masq),
    }
}

pub async fn send(token: &str, message: &RMessage , content: &str) {

    let reply = RReplies {
        id: message._id.to_string(),
        mention: false,
    };
    let payload2 = RMessagePayload {
        content: Some(String::from(content)),
        replies: Some(vec![reply]),
          attachments: None,
          masquerade: None
    };

    rev_send(token, &message.channel, payload2).await;
}


pub fn reply_from(input: &RMessage) -> RReplies {
    
    RReplies {
        id: input._id.to_owned(),
        mention: false,
    }
}

pub fn link_to_embed(input: &str) -> String {
    format!("[]({input})")
}