

use crate::{lib::{conf::Auth, message::{RMessage, Masquerade, RMessagePayload, RReplies}}, rev_x::{rev_send, rev_del_2, rev_kick, rev_fetch_channel}, plugins::lreywen::send};
use crate::plugins::lreywen::reply_from;
#[derive(Debug, Clone, Default)]
pub struct Reywen {
    auth: Auth,
    input_message: RMessage,


}

impl Reywen {
    
    pub fn new(auth: Auth, input_message: &RMessage) -> Self {



        let input_message = input_message.to_owned();


        Reywen
        {
            auth, input_message
        }
    }
    
  
    pub async fn send(self, payload: RMessagePayload) -> Self {
    
        
        rev_send(&self.auth.token, &self.input_message.channel, payload).await;
        self
    }
    pub async fn sender(self, content: &str) -> Self {
        

        let payload = RMessagePayload {
            content: Some(String::from(content)),
            attachments: None,
            replies: Some(vec![reply_from(&self.input_message)]),
            masquerade: None,
        };
        rev_send(&self.auth.token, &self.input_message.channel, payload).await;
        self
    }
    pub async fn delete_msg(self, message_id: &str) -> Self {
        rev_del_2(&self.auth.token, message_id, &self.input_message.channel).await;
        self
    }

    pub async fn member_kick(self, user: &str) -> Self {

        let server = rev_fetch_channel(&self.input_message.channel, &self.auth.token).await.unwrap().server;
        let payload = format!("**Kicking {}**", user);
        let user = user.replace(['@', '<', '>'], "");

        tokio::join!(
                rev_kick(&self.auth.token, &user, &server),
                send(&self.auth.token, &self.input_message, &payload),
                );
        self
    }
}

impl RMessagePayload {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn content(mut self, content: &'static str) -> Self {
        self.content = Some(content.to_string());
        self
    }
    
    pub fn masquerade(mut self, masq: Masquerade) -> Self {
        self.masquerade = Some(masq);
        self
        
    }
    
    pub fn reply(mut self, input: &RMessage) -> Self {
        self.replies = Some(vec![reply_from(input)]);
        self
    }
    
    pub fn replies(mut self, replies: Vec<RReplies>) -> Self {
        self.replies = Some(replies);
        self
    }
}




pub async fn oop_main(auth: Auth, input_message: &RMessage) {


    // reywen is the main method for this bot,
    // it can be used to interact with the API both normally and with abstrated methods
    let client = Reywen::new(auth, input_message);

    // crash condition checks for a list of undesireable
    // the second variable is an optional 'command prefix',
    if crash_condition(input_message, Some("?m")) {return};


    // vector of content
    let convec = convec(input_message);


    // matches all possilbe inputs and defines desired results as an str to be sent
    let payload = match convec[1] as &str {
        "?mog" => ":01G7MT5B978E360NB6VWAS9SJ6:",
        "VeiledProduct" => "no",
        "sussing" => "amogus",
        _ => "NoneOption%",
    };
    if payload != "NoneOption%" {
        client.clone().sender(payload).await;
    };

    // this is an example of send (not sender)
    // its lower level, giving you more control over reywen while
    // requiring more boilerplate.
    // construct a payload and then send it!

    let payload = RMessagePayload::new().content("world").reply(input_message);

    if convec[1] == String::from("hello!") {

        client.clone().send(payload).await;
    };
    // the available fields on RMessagePayload are the same as the API request requirements,
    // with additional fields for abstration




    // there are two main ways to use delete_msg, automated and command line
    // both of which would most often be used for moderation
    // there are alternate uses but they will not be covered here

    // convec way
    if convec.len() > 2 {
        if convec[1] == "?del" {
            client.clone().delete_msg(convec[2]).await;
        };
    };

    // input_message way
    // this example is a banned word list
    let wordlist = vec!["badword", "anotherbadword!!", "illegalwords"];

    for x in wordlist.iter() {
        if convec.contains(x) {
            client.clone().delete_msg(&input_message._id).await;

        };
    };

    // kicking a user can simply be done by specifying the user ID
    //client.member_kick("USERID")
    // use this method with care, while i can't stop anyone i'd rather that reywen
    // is not used for spamming or abuse on revolt

    // simutanious actions
    // for network applications such as reywen multithreading is not needed
    // instead tasks can run at the same time on the same thread with tokio

    if &convec[1] as &str == "?async" {
        tokio::join!(

                client.clone().sender("hey!"),
                client.delete_msg(&input_message._id),
                );
    };
    // notice the lack of await - its handled by tokio join


    // there are many other methods and more comming soon! but this is enough of an example
    // to get started, enjoy ^^ - Toast

}

