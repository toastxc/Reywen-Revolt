mod rev_curl;
use std::{thread, time};

use rev_curl::{permcheck, rev_history, rev_read, rev_send, sendas, divancheck, man};
#[derive(Debug, Clone)]
struct Data {

    token: String,
    channel: String,
    bot_id: String,
    sudoers: Vec<String>
}



fn main() {

    let data = Data {
    token: "TOKEN".to_string(),
    channel: "CHANNEL".to_string(),
    bot_id: "BOTID".to_string(),
    sudoers: vec!["SUDOER1".to_string(), "SUDOER2".to_string()]
    };


    //users


    let help = "run ?man for help".to_string();

    let sec = time::Duration::from_secs(2);


    // main session

   
    loop {

        // rate limit
        thread::sleep(sec);
      

       let (raw, user) = rev_read(data.token.clone(), data.channel.clone());
       let mut args = raw.split(" ").collect::<Vec<&str>>();
       let mes = args[0];
       
       let sudo = permcheck(user.clone(), data.sudoers.clone());

       if user.clone() == data.bot_id {
           // nothing
       }else if mes.chars().nth(0).unwrap() != '?' {
           // nothing


        // general 
       }else if mes == ("?help".to_string()) {
            println!("sending help");
            rev_send(data.token.clone(), data.channel.clone(), help.clone());
        
        }else if mes == ("?ping".to_string()) {
            println!("PingPong");
            rev_send(data.token.clone(), data.channel.clone(), "Pong!!".to_string());
        
        }else if mes == ("?man".to_string()){
            if args.len() < 2 {
                rev_send(data.token.clone(), data.channel.clone(), man("man".to_string()));
            }else {
                rev_send(data.token.clone(), data.channel.clone(), man(args[1].to_string()));
            };     


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
                rev_send(data.token.clone(), data.channel.clone(), "invalid use of sendas".to_string());
            }else {
                sendas(data.token.clone(), data.channel.clone(), args);
            };
        };
    }
}
