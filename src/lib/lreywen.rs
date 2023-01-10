// an abstraction layer for RevX2
use crate::structs::message::{RMessagePayload, RMessage, RReplies};

use super::rev_x::rev_send;

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

#[allow(dead_code)]
pub fn reply_from(input: &RMessage) -> RReplies {

    RReplies {
        id: input._id.to_owned(),
        mention: false,
    }
}
#[allow(dead_code)]
pub fn lte(input: &str) -> String {
    format!("[]({input})")
}


// if the input message is not usable for reywen then return
#[allow(dead_code)]
pub fn crash_condition(input_message: &RMessage, character: Option<&str>) -> bool {

    if input_message.content.is_none() {
        return true
    };

    let temp_convec: Vec<&str> =  input_message.content.as_ref().unwrap().split(' ').collect::<Vec<&str>>();

    let mut length = 2;

    if character.is_none() {
        length = 1;
    };

    if temp_convec.len() < length {
        return true
    };

   if character.is_some() {
       if temp_convec[0] != character.unwrap() {
           return true
       };
   };
   false
}


#[allow(dead_code)]
pub fn convec(input_message: &RMessage) -> Vec<&str> {
    input_message.content.as_ref().unwrap().split(' ').collect::<Vec<&str>>()
}
