#[path = "./rev_x.rs"]
mod rev_x;
use rev_x::*;

use std::str::from_utf8;
use rand::Rng;
use std::process::Command;



pub fn sendas(token: String, channel: String, args: Vec<&str>) {

    let masq = args[1];
    let mut content = String::new();

    for x in 0..args.len() -2  {
        content = content.to_owned() + " "  + args[x + 2]
    };

    //println!("masq: {}\ncontent: {}", masq, content);

// ###################################### curl

    let api = "https://api.revolt.chat/channels/".to_owned() + &channel + "/messages";
    let token = "x-bot-token: ".to_owned() + &token;

    // RNG

    let mut random = rand::thread_rng();
    let idem: i16 = random.gen();
    let idem_print = "Idempotency-Key: ".to_owned() + &idem.to_string();


       let content_print2 = r#"{
    "content": ""#.to_owned() + &content + r#"",
    "masquerade":
    {
        "name": ""# + masq + r#"",
        "avatar": "https://toastxc.xyz/TXCS/"# + masq + r#".jpg"
    }}"#;



    let send = Command::new("curl")
        .args([
               "-sS", "-X", "POST", &api,
              "-H", &token,
              "-H", &idem_print,
              "-H", "Content-Type: application/json",
              "--data", &content_print2
        ])
        .output()
        .expect("failed to run");

    let stdout = from_utf8(&send.stdout).unwrap().to_string();
    let stderr = from_utf8(&send.stderr).unwrap().to_string();

    println!("{}\n{}", stdout, stderr);
}


pub fn divancheck(server: String) -> String {

    let curl = Command::new("curl")
        .arg("https://api.mcsrvstat.us/2/".to_owned() + &server)
        .output()
        .expect("failed to run");


     let curlout = from_utf8(&curl.stdout).unwrap().to_string();
     //let curlerr = from_utf8(&curl.stderr).unwrap().to_string();

     let returner = ajson::get(&curlout, "online").unwrap().to_string();

     return returner
}
pub fn mog(input: String) -> String {
    
    

    let mogus = ":01G7MT5B978E360NB6VWAS9SJ6:";


    let mut input_i32 = input.parse::<i32>();

    let mut err: bool = true;

    for x in 0..20 {
        if input_i32 == Ok(x) {
            err = false;
            input_i32 = Ok(input_i32.unwrap() );

        }else {
        };
    };

    if err == true {
        return "you fool! :trol:".to_string()
    };


    let mut returner = String::new();

    
    for x in 0..input_i32.unwrap() {
        returner += mogus
    };

    
    return returner
}


pub fn man(input: String) -> String {

    let mut returner = "";


    // general

    let man =
        "**help**  \\n`meme:` beans, mog\\n`utility:` mc, info, sendas, ping";

      let info =
        "#2 Reywen\\nA meme and utility bot with no real purpose\\source: https://github.com/toastxc/Reywen-Revolt";


    // meme

       let sendas =
           "sendas uses masqurade to replace a sentence with a different profile picture and name, the pfps are pulled from toastxc.xyz and corospond to the name given\\n```text\\n?sendas woof hewo!";

        let mog =
            "**mog**\\nsends a given number of mogs\\ntext\\n?mog 4";

        let beans =
            "**beans\\nsends bean memes\\n?beans";

        let info =
            "#2 Reywen\\nA meme and utility bot with no real purpose\\source: https://github.com/toastxc/Reywen-Revolt";

    // utility

        let ping =
            "**ping** - simple ping test for bot latency\\ntext```\\n?ping";

        let mc =
            "**mc** - checks the status of a minecraft server\\nexample: \\n```text\\n?mc toastxc.xyz";


    match &input as &str {

        //general
        "info" | "information" => returner = info,
        "help" | "man" | "manual" => returner = man,
        //meme
        "sendas" => returner = sendas,
        "mog" | "sus" | "amogus" => returner = mog,
        "beans" | "bean" => returner = beans,
        //utility
        "mc" | "minecraft" | "divancheck" | "mcquery" => returner = mc,
        "ping" => returner = ping,
        "sendas" => returner = sendas,
        // else
            &_ => returner = man

    };

    return returner.to_string()

}
