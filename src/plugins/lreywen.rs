// a library for high level non essential functions

use crate::{
    Auth, lib::message::{RMessage, RReplies, RMessagePayload, Masquerade},
};
use crate::rev_x::*;


pub async fn bash_masq(out: String) -> RMessagePayload {

      let masq = Masquerade {
        name: Some("ReyShell".to_string()),
        avatar: Some("https://toastxc.xyz/TXCS/reyshell.png".to_string()),
        colour: None,
    };

    RMessagePayload {
        content: Some(out),
        attachments: None,
        replies: None,
        masquerade:  Some(masq)
    }

}

pub async fn send(auth: Auth, message: RMessage, content: String) {

    let reply = RReplies {
        id: message._id.clone(),
        mention: false,
    };
    let payload2 = RMessagePayload {
        content: Some(content),
        replies: Some(vec![reply]),
          attachments: None,
          masquerade: None
    };

    rev_send(auth, message, payload2).await;

}
// masq wrapper for rev_send
pub async fn sendas(auth: Auth, message: RMessage, content_vec: Vec<&str>) {

    if content_vec.len() < 3 {
        send(auth, message, "invalid use of sendas".to_string()).await;
        return
    };
    //let from = message._id.clone();
    let masq = content_vec[1];
    let mut content = String::new();
    //content = "placeholder".to_string();

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

    rev_send(auth.clone(), message.clone(), returner).await;
    rev_del(auth.clone(), message.clone()).await;
}

