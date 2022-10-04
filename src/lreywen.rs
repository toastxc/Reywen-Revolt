// a library for high level functions

use crate::lib::{
    auth::Auth,
    message::{RMessage, RReplies, RMessagePayload},
};

use crate::rev_x::rev_send;

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
