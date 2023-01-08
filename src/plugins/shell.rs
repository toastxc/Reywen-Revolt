// external
use std::process::Command;
use serde::{Serialize, Deserialize};

// internal
use crate::{fs_str, structs::{message::{RMessage, Masquerade, RMessagePayload}, auth::Auth}, lib::{rev_x::{sudoer}, lreywen::{crash_condition, convec}, oop::Reywen}};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShellConf {
    pub enabled: bool,
    pub whitelist_sudoers: bool,
    pub channel_only: bool,
    pub channel: String,
}

pub async fn shell_main(auth: Auth, input_message: &RMessage) {

    // import config
    let conf = fs_str("config/shell.json")
        .expect("failed to read config/shell.json\n{e}");

    let shell: ShellConf = serde_json::from_str(&conf)
            .expect("Failed to deser shell.json");

    if !shell.enabled {
        return
    };

    let client = Reywen::new(auth.clone(), input_message);


    let masq = Masquerade::new()
        .name("Reyshell")
        .avatar("https://toastxc.xyz/TXCS/reyshell.png");

    let payload = RMessagePayload::new()
        .masquerade(masq);

    if crash_condition(input_message, Some("?/")) {return};

    // due to how dangerous shell commands are, there needs to be security checks

    if shell.channel_only && shell.channel != input_message.channel {
        return
    };
    if shell.whitelist_sudoers && !sudoer(&input_message.author, "SHELL", &auth.sudoers) {
        client.send(payload.content("**Only sudoers allowed**")).await;
        return
    };

    let convec = convec(input_message);

    let mut content_min1 = String::new();

    for x in 0..convec.len() -1 {
        content_min1 += &format!("{} ", convec[x + 1])
    };

    bash_exec(client, convec, payload).await;
}

pub async fn bash_exec(client: Reywen, convec: Vec<&str>, payload: RMessagePayload) {


    // shell

    let mut com = Command::new(convec[1]);

    for x in 0..convec.len() -2 {
        com.arg(convec[x+2]);
    };

    if let Err(e) = com.output() {

        client.send(payload.content(&e.to_string())).await;
        return
    };


    let stdout = com.output().expect("error with stdout").stdout;
    let stderr = com.output().expect("error with stdout").stderr;

    let out = String::from_utf8_lossy(&stdout) + String::from_utf8_lossy(&stderr);


    if out.chars().count() <= 1000 {

        client.send(payload.content(&format!("```text\n{out}"))).await;

    }else {

        bash_big_msg(out.to_string(), client, payload).await;

        };

}

pub async fn bash_big_msg(out: String, client: Reywen, payload: RMessagePayload) {

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

        client.clone().send(payload.clone().content(&format!("```\\n\\n{current}"))).await;

        current = String::new();

    };

    if c > 0 {
        for _ in 0..c {

            current += &vec[( iter - 1) as usize].to_string();
            iter += 1;
        };

          current = format!("```\\n\\n{current}");

        client.send(payload.content(&current)).await;

    };
    println!();

}

pub fn convert(a: i32) -> (i32, i32, i32){

    if a < 1000 {
        return (1, a, 0)
    };
   
    (a / 1000, 1000,  a % 1000)
}