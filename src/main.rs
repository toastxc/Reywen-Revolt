mod rev_x;
use std::{thread, time};
use std::process::Command;
use std::str::from_utf8;

use rev_x::*;
#[derive(Debug, Clone)]

struct Data {

    token: String,
    bot_id: String,
    channel: String,
    sudoers: Vec<String>
}



fn main() {

    // credentials
       let data = Data {
        token: "".to_string(),
        bot_id: "".to_string(),
        channel: "".to_string(),
        sudoers: vec!["".to_string()],
    };

       // wordban
       let wordlist = vec!["example".to_string(), "example2".to_string()];
       let wordban = true;

       // credentials check
    if data.token == "" {
        println!("bot token required for functionality");
            return
    }else if data.bot_id == "" {
        println!("bot id required for functionality");
    }else if data.channel == "" {
        println!("channel required for functionality");
    }else if data.sudoers[0] == "" {
        println!("WARN: no sudoers\nno users are able to run privileged  commmands")
    }else {
        println!("valid credentials, starting bot...");
    };

    let sec = time::Duration::from_secs(2);

    // main session

   
    loop {

        // rate limit
        thread::sleep(sec);
     

       let (content, user, id) = rev_read(data.token.clone(), data.channel.clone());

       let mut out = String::new();

       for x in 0..content.chars().count() {

           if content.chars().nth(x) == Some('\n') {
               out = out + "\\n";
           }else {
               out = out + &(content.chars().nth(x).unwrap().to_string());
           };
       };

       let args = out.split(' ').collect::<Vec<&str>>();
       let mes = args[0];


       let sudo = permcheck(user.clone(), data.sudoers.clone());

       if user.clone() == data.bot_id {
           // nothing
       }else if mes.chars().nth(0).unwrap() != '?' {
           // nothing


        // general 
       }else if mes == ("?help".to_string()) {
            println!("sending help");
            rev_send(data.token.clone(), data.channel.clone(), man("man".to_string()));

        
        }else if mes == ("?ping".to_string()) {
            println!("PingPong");
            rev_send(data.token.clone(), data.channel.clone(), "Pong!!".to_string());
        
        }else if mes == ("?man".to_string()){
            if args.len() < 2 {
                rev_send(data.token.clone(), data.channel.clone(), man("man".to_string()));
            }else {
                rev_send(data.token.clone(), data.channel.clone(), man(args[1].to_string()));
            };

            }else if mes == ("?sudo".to_string()) {
                println!("sudo check");
                rev_send(data.token.clone(), data.channel.clone(), sudo.to_string());
            


            // TXC services 
        }else if mes == ("?mc".to_string()) {
            println!("running mc check");
            rev_send(data.token.clone(), data.channel.clone(), divancheck(args[1].to_string()));
       

            // sudoers

        }else if sudo == false {
            rev_send(data.token.clone(), data.channel.clone(), "invalid permissons".to_string())
        
        }else if mes == ("?killbot".to_string()) {
            if args.len() == 2 {
                if args[1].to_string() == "--confirm" {
                    println!("recived kill switch, stopping bot");
                    rev_send(data.token.clone(), data.channel.clone(), "killing bot...".to_string());
                    return
                };
            }else {
                rev_send(data.token.clone(), data.channel.clone(), "run `?killbot --confirm` to confirm".to_string())
            }; 

        }else if mes == "?sendas".to_string() {
            if args.len() < 3 {
                rev_send(data.token.clone(), data.channel.clone(), "**Options**\\ncheese, joe_biden, bingus, woof, walter, **Syntax**\\n```text\\n?sendas <name> <content>".to_string());
            }else {
                 rev_del(data.token.clone(), data.channel.clone(), id.to_string());

                 println!("{:?}", args);
                sendas(data.token.clone(), data.channel.clone(), args);

            };

        }else if mes == "?delete".to_string() {
            if args.len() < 2 {
                rev_send(data.token.clone(), data.channel.clone(), "invalid use of delete".to_string());
            }else {
                rev_del(data.token.clone(), data.channel.clone(), id.to_string());
    
            };
        };

       // wordban
       if wordban == true {
           rev_wordban(data.token.clone(), data.channel.clone(), wordlist.clone());
       };
    


    };


}
