use crate::{Auth, RMessage, rev_send, sudocheck, reyshell_masq};
use std::process::Command;
use crate::fs_str;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShellConf {
    pub enabled: bool,
    pub whitelist_sudoers: bool,
    pub bash_sudo: bool,
    pub channel: SocConf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SocConf {
    pub enabled: bool,
    pub channel: String,
}

pub async fn shell_main(details: Auth, message: &RMessage) {

    let conf = fs_str("config/shell.json")
        .expect("failed to read config/shell.json\n{e}");

    let shell: ShellConf = serde_json::from_str(&conf)
            .expect("Failed to deser shell.json");

    let content = match message.content {
        None => return,
        Some(ref m) => m,
    };
 
    // initalize variables
    let (auth, soc) = (details.clone(),  shell.channel.clone());
    let content_vec =  content.split(' ').collect::<Vec<&str>>();


    

    // perm check 
    if !shell.enabled {
        return
    }else if message.content.is_none() {
        return
   }else if soc.enabled && soc.channel != message.channel {
       return
   }else if content_vec[0] != "?/" {
       return
   }else if content_vec.len() <= 1 {
        return
    }else if shell.whitelist_sudoers && sudocheck(&message.author, "SHELL", &auth.sudoers) {
        rev_send(&auth.token, &message.channel, reyshell_masq("**Only sudoers allowed**")).await;
        return
    };

    let mut content_min1 = String::new();

    for x in 0..content_vec.len() -1 {
        content_min1 += &format!("{} ", content_vec[x + 1])
    };

    bash_exec(content_vec, details.clone(), message.clone()).await;
               

}

pub async fn bash_exec(input: Vec<&str>, details: Auth, message: RMessage) {

    // shell

    let mut com = Command::new(input[1]);

    for x in 0..input.len() -2 {
        com.arg(input[x+2]);
    };

    if let Err(e) = com.output() {
        rev_send(&details.token, &message.channel, reyshell_masq(&e.to_string())).await;
        return};


    let stdout = com.output().expect("error with stdout").stdout;

    let out = String::from_utf8_lossy(&stdout);

    if out.chars().count() <= 1900 {        

        rev_send(&details.token, &message.channel, reyshell_masq(&format!("```text\n{out}"))).await

    }else {

        bash_big_msg(out.to_string(), details.clone(), message.clone()).await;

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
            iter += 1;
        };

        //let payload = reyshell_masq(format!("```\\n\\n{current}"));

        rev_send(&auth.token, &message.channel, reyshell_masq(&format!("```\\n\\n{current}"))).await;

        current = String::new();

    };

    if c > 0 {
        for _ in 0..c {

            current += &vec[( iter - 1) as usize].to_string();
            iter += 1;
        };

          current = format!("```\\n\\n{current}");

        let payload = reyshell_masq(&current);

        rev_send(&auth.token, &message.channel, payload).await;

    };
    println!();

}

pub fn convert(a: i32) -> (i32, i32, i32){

    if a < 1900 {
        return (1, a, 0)
    };
   
    (a / 1900, 1900,  a % 1900)
}


