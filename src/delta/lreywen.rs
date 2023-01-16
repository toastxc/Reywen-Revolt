// an abstraction layer for RevX2 - functionality has been moved from here to oop

use crate::quark::delta::message::{RMessage, RReplies};

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
        return true;
    };

    let temp_convec: Vec<&str> = input_message
        .content
        .as_ref()
        .unwrap()
        .split(' ')
        .collect::<Vec<&str>>();

    let mut length = 2;

    if character.is_none() {
        length = 1;
    };

    if temp_convec.len() < length {
        return true;
    };

    if character.is_some() && character != Some(temp_convec[0]) {
        return true;
    };
    false
}

#[allow(dead_code)]
pub fn convec(input_message: &RMessage) -> Vec<&str> {
    input_message
        .content
        .as_ref()
        .unwrap()
        .split(' ')
        .collect::<Vec<&str>>()
}
