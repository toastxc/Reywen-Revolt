// an abstraction layer for RevX2
use crate::structs::message::{RMessagePayload, RMessage, RReplies, Masquerade};

use super::rev_x::rev_send;


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


// if the input message is not usable for reywen then return
pub fn crash_condition(input_message: &RMessage, character: Option<&str>) -> bool {

    if input_message.content.is_none() {
        return true
    };

    let temp_convec: Vec<&str> =  input_message.content.as_ref().unwrap().split(' ').collect::<Vec<&str>>();

    let mut length = 1;

    if character.is_none() {
        length = 2;
    }

    if temp_convec.len() < length {
        return true
    };

    if character.is_some() {
        if temp_convec[0] != character.unwrap() {
            return true
        };
    }
    false
}



pub fn convec(input_message: &RMessage) -> Vec<&str> {
    input_message.content.as_ref().unwrap().split(' ').collect::<Vec<&str>>()
}