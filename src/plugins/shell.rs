use crate::{MainConf, RMessage, send, bash_masq, rev_send, sudocheck};
use std::process::Command;
pub async fn shell_main(details: MainConf, message: RMessage) {

    // initalize variables

    let user = message.author.clone();
    let (auth, shell, soc) = (details.auth.clone(), details.shell.clone(), details.shell.shell_channel.clone());
    let content_vec =  message.content.as_ref().expect("failed to split vec").split(' ').collect::<Vec<&str>>();


    let sudoer = sudocheck(message.author.clone(), details.auth.clone()).await;
    // perm check 

    if details.shell.enabled == false {
        return
    }else if message.content == None {
        return
   }else if soc.enabled == true && soc.channel != message.channel {
       return
   }else if content_vec[0] != "?/" {
       return
   }else if content_vec.len() <= 1 {
        return
    }else if shell.whitelist_sudoers == true && sudoer != true {
        rev_send(details.auth, message, bash_masq("only sudoers allowed".to_string()).await).await;
        return
    };



    let mut content_min1 = String::new();

    for x in 0..content_vec.len() -1 {
        content_min1 += &format!("{} ", content_vec[x + 1])
    };

    bash_exec(content_vec, details.clone(), message.clone()).await;
               

}

pub async fn bash_exec(input: Vec<&str>, details: MainConf, message: RMessage) {

    // shell

    let mut com = Command::new(input[1]);

    for x in 0..input.len() -2 {
        com.arg(input[x+2]);
    };


    match com.output() {
        Err(e) => {
            rev_send(details.auth, message, bash_masq(e.to_string()).await).await;
            return},
        Ok(_) => {},
    };

    let stdout = com.output().expect("error with stdout").stdout;



    let out = String::from_utf8_lossy(&stdout);

    println!("{:?}", out);
    if out.chars().count() <= 2000 {        

        rev_send(details.auth, message, bash_masq(format!("```\n{out}```")).await).await

    }else {


        let out_vec = out.split('\n').collect::<Vec<&str>>();

        for x in 0..out_vec.len() {
            send(details.auth.clone(), message.clone(), out_vec[x].to_string()).await
        };
    };

}


