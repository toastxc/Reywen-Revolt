use crate::{MainConf, RMessage, bash_masq, rev_send, sudocheck, Auth};
use std::process::Command;
pub async fn shell_main(details: MainConf, message: RMessage) {

    // initalize variables
    let (auth, shell, soc) = (details.auth.clone(), details.shell.clone(), details.shell.shell_channel.clone());
    let content_vec =  message.content.as_ref().expect("failed to split vec").split(' ').collect::<Vec<&str>>();


    let sudoer = sudocheck(message.author.clone(), auth.clone()).await;
    

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
        rev_send(auth, message, bash_masq("**Only sudoers allowed**".to_string()).await).await;
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

    if out.chars().count() <= 2000 {        

        rev_send(details.auth, message, bash_masq(format!("```text\n{out}")).await).await

    }else {

        bash_big_msg(out.to_string(), details.auth.clone(), message.clone()).await;

        };

}

pub async fn bash_big_msg(out: String, auth: Auth, message: RMessage, ) {

    let vec: Vec<char> = out.chars().collect();

    let (a, b, c) = convert(vec.len() as i32);

    // iterator
    // payload
    // remainder


    let mut current = String::new();
    let mut iter = 0;

    for _ in 0..a {
        for _ in 0..b {

            current += &vec[(iter) as usize].to_string();
            iter = iter + 1;
        };
        print!("{current}");

        current = format!("```\\n\\n{current}");
        let payload = bash_masq(current).await;

        rev_send(auth.clone(), message.clone(), payload).await;


        current = String::new();

    };

    if c > 0 {
        for _ in 0..c {

            current += &vec[( iter - 1) as usize].to_string();
            iter += 1;
        };

          current = format!("```\\n\\n{current}");

        let payload = bash_masq(current).await;

        rev_send(auth.clone(), message.clone(), payload).await;

    };

    print!("\n");

}

pub fn convert(a: i32) -> (i32, i32, i32){

    if a < 1900 {
        return (1, a, 0)
    };
   
    return (a / 1900, 1900,  a % 1900);
}

