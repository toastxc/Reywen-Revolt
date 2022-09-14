mod rev_x;

use rev_x::*;
use ajson::*;
use url::Url;
use tungstenite::{connect, Message};

#[derive(Debug, Clone)]
struct Data {

    token: String,
    bot_id: String,
    sudoers: Vec<String>
}



fn main() {

    // credentials


       let data = Data {
        token: "".to_string(),
        bot_id: "".to_string(),
        sudoers: vec!["".to_string()],
    };


       // wordban
       let wordlist = vec!["example".to_string()];
       let wordban_bool = false;

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

 
        
        let raw = socket.read_message().expect("Error reading message").to_string();


        let mes_type = ajson::get(&raw, "type").unwrap().to_string();
      
        

        if mes_type == "Message" {

        
            let mut content = ajson::get(&raw, "content").unwrap().to_string();
            let channel = ajson::get(&raw, "channel").unwrap().to_string();
            let author = ajson::get(&raw, "author").unwrap().to_string();
            let id = ajson::get(&raw, "_id").unwrap().to_string();
        

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


            // @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

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
                       

                    }else if content == "?kick" {
                        if sudo == false {
                            rev_send(data.token.clone(), channel.clone(), "you require sudo for ?kick".to_string());
                        }else if args.len() < 2 {
                                rev_send(data.token.clone(), channel.clone(), "invalid use of ?kick".to_string());
                            }else {
                                rev_kick(data.token.clone(), channel.clone(), content2.clone());
                                println!("kicking {}", content2.clone());
                            };
                        }; 

                    }
                            
                    if wordban_bool == true {

                        wordban(data.token.clone(), channel.clone(), wordlist.clone(), raw);


        

                    };
                };
            };
        };
    };
}


