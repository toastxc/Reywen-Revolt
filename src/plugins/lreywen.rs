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
        id: message._id.clone(),
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
// masq wrapper for rev_send this is outdated and has been replaced by the plugin plural
pub async fn sendas(token: &str, message: &RMessage, content_vec: &Vec<&str>) {

    if content_vec.len() < 3 {
        send(token, message, "invalid use of sendas").await;
        return
    };
    let masq = content_vec[1];
    let mut content = String::new();

    let link = match masq {
        "bingus"    | "cheese"  | "dad" |
        "deeznuts"  |  "insert" | "joe_biden" |
        "valence"   | "walter"  | "woof" => format!("https://toastxc.xyz/TXCS/{masq}.jpg"),
        _ => String::from("https://toastxc.xyz/TXCS/default.png")
    };

    for x in 0..content_vec.len() -2 {
        content += &format!(" {}", content_vec[x + 2]);
    };

    let masq_s = Masquerade {
        name: Some(masq.to_string()),
        avatar: Some(link),
        colour: None,
    };


    let replier = rev_convert_reply(message.replies.clone()).await;

    let returner = RMessagePayload {
          content: Some(content),
          replies: replier,
          attachments: None,
          masquerade: Some(masq_s)
    };

    tokio::join!( 
        rev_send(token, &message.channel, returner),
        rev_del(token, message),
    );
}

