mod rev_x;
mod reywen;

// Reywen Libraries
use rev_x::*;
use reywen::*;

// websocket
use url::Url;
use tungstenite::{connect, Message};

// fs 
use std::io::Read;
use std::fs::*;

// serde
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

// time
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Auth {

    token: String,
    bot_id: String,
    sudoers: Vec<String>,

    wordban: bool,
    wordlist: Vec<String>

}


#[derive(Serialize, Deserialize, Debug, Clone)]
struct Messages {
                r#type: String,
                _id: String,
                nonce: String,
                channel: String,
                author: String,
                content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Messages_at {
                r#type: String,
                _id: String,
                nonce: String,
                channel: String,
                author: String,
                content: String,
        // additional
                attachments: Vec<Attachment>

                
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Attachment {
    _id: String,
    tag: String,
    filename: String,
    metadata: Metadata,
    content_type: String,
    size: u32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Metadata {
    r#type: String,
    width: u32,
    height: u32,
}


// debug structs - used for determining type
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Attach {
     attachments: Vec<Attachment>
}


fn message2struct(json: String) -> Result<Messages> {
    
    let out: Messages = serde_json::from_str(&json)?;

    Ok(out)


}

fn conf_serde(json: String) -> Result<Auth> {

        let conf: Auth = serde_json::from_str(&json)?;

        Ok(conf)
}


fn main()  {


    let mut data_str = String::new();

    let mut config_json = File::open("config.json")
        .expect("File not found");

    config_json.read_to_string(&mut data_str)
        .expect("Error while reading file");


     let data = conf_serde(data_str.to_string()).unwrap();


       // credentials check
    if data.token == "" {
        println!("bot token required for functionality");
            return
    }else if data.bot_id == "" {
        println!("bot id required for functionality");
    }else if data.sudoers[0] == "" {
        println!("WARN: no sudoers\nno users are able to run privileged  commmands")
    }else {
        println!("valid credentials, starting bot...");
    };

    // new auth
    
    let url = "wss://ws.revolt.chat/?format=json&version=1&token=".to_owned() + &data.token;

    let (mut socket, response) = connect(Url::parse(&url).unwrap()).expect("Can't connect");


    loop { 
        
         let ping = r#"{
    "type": "Ping",
    "data": 0
}"#;

        
        socket.write_message(Message::Text(ping.to_string()));

        let raw = socket.read_message().expect("Error reading message").to_string();

        let mes_type = ajson::get(&raw, "type").unwrap().to_string();
      

        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        if mes_type == "Authenticated" {

            let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        

        }else if mes_type == "Message" {

            println!("raw\n{raw}\n\n\n");


            let raw_json = message2struct(raw).unwrap();

            let (content, channel, author, id) = (raw_json.content, raw_json.channel, raw_json.author, raw_json._id);

            let mut out = String::new();

            for x in 0..content.chars().count() {

                if content.chars().nth(x) == Some('\n') {
                    out = out + "\\n";

                }else if content.chars().nth(x) == Some('\\') {
                    out = out + "\\\\"
                }else {

                    out = out + &(content.chars().nth(x).unwrap().to_string());
                };
            };
            let args = out.split(' ').collect::<Vec<&str>>();

            let content = args[0];

            let mut content2 = String::new();
            if args.len() >= 2 {
                content2 = args[1].to_string();
            };


            let sudo = permcheck(author.clone(), data.sudoers.clone());

            if author.clone() != data.bot_id {

                if content.chars().count() >= 2 {


                    if content.chars().nth(0).unwrap() == '?' {

                    // main

                    if content == "?help" {
                        rev_send(data.token.clone(), channel.clone(), man("man".to_string()));

                    }else if content == "?ping" {
                        rev_send(data.token.clone(), channel.clone(), "Pong!!".to_string());

                    }else if content == "?man" {

                        if args.len() < 2 {
                            rev_send(data.token.clone(), channel.clone(), man("man".to_string()));
                        }else {
                            rev_send(data.token.clone(), channel.clone(), man(content2));
                        };
                    }else if content == ("?sudo".to_string()) {
                        rev_send(data.token.clone(), channel.clone(), sudo.to_string());

                    }else if content == ("?mc".to_string()) {
                            rev_send(data.token.clone(), channel.clone(), divancheck(args[1].to_string()));

                    }else if content == "?sendas" {
                        if args.len() < 3 {
                            rev_send(data.token.clone(), channel.clone(), "**Options**\\ncheese, joe_biden, bingus, woof, walter, **Syntax**\\n```text\\n?sendas <name> <content>".to_string());
                        }else {
                            rev_del(data.token.clone(), channel.clone(), id.to_string());
                            sendas(data.token.clone(), channel.clone(), args);
                        };
                                            

                    


                    }else if content == "?purge" {
                        if sudo == false {
                            rev_send(data.token.clone(), channel.clone(), "you require sudo for ?purge".to_string());
                        }else 
                            if args.len() < 2 {
                            rev_send(data.token.clone(), channel.clone(), "invalid use of ?purge".to_string());
                            }else {
                                purge(data.token.clone(), channel.clone(), content2);
                            };
                    
                    }else if content == "?mog" {
                        if args.len() < 2 {
                            rev_send(data.token.clone(), channel.clone(), mog("1".to_string()));
                        }else if args.len() >= 2 {
                                rev_send(data.token.clone(), channel.clone(), mog(content2));

                   
                        };
                        };
                    };
                            
                    if data.wordban == true {

                        wordban(data.token.clone(), channel.clone(), data.wordlist.clone(), content.to_string(), id.to_string());


        

                    };
                };
            };
        };

            let elapsed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

     
            
            if (elapsed - start).as_secs() > 1 {
                println!("WARN: thread processing took {:?}", elapsed - start);
            };

    };

}
