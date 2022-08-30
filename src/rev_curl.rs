use std::process::Command;
use std::str::from_utf8;
use rand::Rng;


pub fn rev_del(token: String, channel: String, target: String)  {

    let api = "https://api.revolt.chat/channels/".to_owned() + &channel + "/messages/" + &target;
    let token = "x-bot-token: ".to_owned() + &token;

    let curl =  Command::new("curl")
        .args([
               "-sS", "-X", "DELETE", &api,
              "-H", &token,

        ])
        .output()
        .expect("failed to run");


    let curl_stdout = from_utf8(&curl.stdout).unwrap().to_string();
    let curl_stderr = from_utf8(&curl.stderr).unwrap().to_string();

}

pub fn permcheck(user: String, sudoers: Vec<String>) -> bool {

    for x in 0..sudoers.len() {

        if user == sudoers[x] {
            return true
        };
    };
    return false
}
pub fn rev_history(token: String, channel: String, mut numget: i32) -> (Vec<String>, Vec<String>){

    struct Returner {

        content: Vec<String>,
        author: Vec<String>,
    }

    let mut n = Returner {
        content: vec![],
        author: vec![],
    };

    let api = "https://api.revolt.chat/channels/".to_owned() + &channel + "/messages";
    let token = "x-bot-token: ".to_owned() + &token;

    let send = Command::new("curl")
        .args([
               "-sS", "-X", "GET", &api,
              "-H", &token,
              "-H", "Content-Type: application/json",
        ])
        .output()
        .expect("failed to run");


    let send_out1_stdout = from_utf8(&send.stdout).unwrap().to_string();
    let send_out1_stderr = from_utf8(&send.stderr).unwrap().to_string();


     let mut list = ajson::get(&send_out1_stdout, "#").unwrap().to_string();

    let list2 = list.parse::<i32>().unwrap();



    if numget > list2 {

        println!("invalid input for rev_history\nrequested {} messages but only {} found\n", numget, list);
        //return (vec!["no".to_string()], vec!["no".to_string()])

        numget = list2;
    }

    let mut x = 0;

        for _x in 0..numget {
            let iter1 = &(x.to_string() + ".content");
            let iter2 = &(x.to_string() + ".author");

            n.content.push("<placeholder>".to_string());
            n.author.push("<placeholder>".to_string());

            n.content[x] = ajson::get(&send_out1_stdout, iter1).unwrap().to_string();
            n.author[x] = ajson::get(&send_out1_stdout, iter2).unwrap().to_string();




             x = x + 1;
        }
            return (n.content, n.author);


        let failed: Vec<String> = vec!["no".to_string()];
        let failed2: Vec<String> = vec!["no".to_string()];


        return (failed, failed2)

}

pub fn rev_read(token: String, channel: String) -> (String, String, String){


    let api = "https://api.revolt.chat/channels/".to_owned() + &channel + "/messages";
    let token = "x-bot-token: ".to_owned() + &token;



    let send = Command::new("curl")
        .args([
               "-sS", "-X", "GET", &api,
              "-H", &token,
              "-H", "Content-Type: application/json",
        ])
        .output()
        .expect("failed to run");


    let send_out1_stdout = from_utf8(&send.stdout).unwrap().to_string();
    let send_out1_stderr = from_utf8(&send.stderr).unwrap().to_string();

   //println!("REVOLT:\nstdout:\n{}\nstderr:\n{}", send_out1_stdout, send_out1_stderr);

   let content = ajson::get(&send_out1_stdout, "0.content").unwrap().to_string();
   let author =  ajson::get(&send_out1_stdout, "0.author").unwrap().to_string();
   let id = ajson::get(&send_out1_stdout, "0._id").unwrap().to_string();

   return (content, author, id)


}

pub fn rev_search(token: String, channel: String, content: String, limit: i8) {

    let api = "https://api.revolt.chat/channels/".to_owned() + &channel + "/search";
    let token = "x-bot-token: ".to_owned() + &token;


    //  DO NOT LINT 
    let content_print = r#"{
  "query": ""#.to_owned() + &content + r#"",
  "limit": "# + &limit.to_string() + r#",
  "sort": "Relevance",
  "include_users": false"# + r#"
}"#;

println!("{}", content_print);


    let send = Command::new("curl")
        .args([
               "-sS", "-X", "POST", &api,
              "-H", &token,
              "-H", "Content-Type: application/json",
              "--data", &content_print
        ])
        .output()
        .expect("failed to run");


    let send_out1_stdout = from_utf8(&send.stdout).unwrap().to_string();
    let send_out1_stderr = from_utf8(&send.stderr).unwrap().to_string();

    println!("REVOLT:\nstdout:\n{}\nstderr:\n{}", send_out1_stdout, send_out1_stderr);

}

pub fn rev_send(token: String, channel: String, content: String) {


    let api = "https://api.revolt.chat/channels/".to_owned() + &channel + "/messages";
    let token = "x-bot-token: ".to_owned() + &token;


    // RNG


    let mut random = rand::thread_rng();
    let idem: i16 = random.gen();
    let idem_print = "Idempotency-Key: ".to_owned() + &idem.to_string();


    //  DO NOT LINT 
    let content_print = r#"{
  "content": ""#.to_owned() + &content + r#""
}"#;


println!("{}", content_print);

    let send = Command::new("curl")
        .args([
               "-sS", "-X", "POST", &api,
              "-H", &token,
              "-H", &idem_print,
              "-H", "Content-Type: application/json",
              "--data", &content_print
        ])
        .output()
        .expect("failed to run");


    let send_out1_stdout = from_utf8(&send.stdout).unwrap().to_string();
    let send_out1_stderr = from_utf8(&send.stderr).unwrap().to_string();

   //println!("REVOLT:\nstdout:\n{}\nstderr:\n{}", send_out1_stdout, send_out1_stderr);

}

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
     let curlerr = from_utf8(&curl.stderr).unwrap().to_string();

     let returner = ajson::get(&curlout, "online").unwrap().to_string();

     return returner
}

pub fn man(input: String) -> String {

    println!("aaaaaaaaaaaaa {}", input);

    let mc = 
        "**mc** - checks the status of a minecraft server\\nexample: \\n```text\\n?mc hypixel.net";
    let ping =
        "**ping** - simple ping test for bot latency, no parameters";
    let killbot = 
        "killbot closes the bot server, can only be accessed by sudoers";
    let sendas =
        "sendas uses masqurade to replace a sentence with a different profile picture and name, the pfps are pulled from toastxc.xyz and corospond to the name given\\nas this feature is still experimental it can only be accessed by sudoers";

    let man =
        "**man** - short for manual\\ninformation on commands\\noptions: mc, ping, killbot, sendas";

    if input == "mc" {
        return mc.to_string()
    }else if input == "ping" {
        return ping.to_string()
    }else if input == "killbot" {
        return killbot.to_string()
    }else if input == "sendas" {
        return sendas.to_string()
    }else if input == "man" {
        return man.to_string()
    }else {
        return man.to_string()
    }

}
