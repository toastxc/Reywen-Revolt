use std::process::Command;
use std::str::from_utf8;
use rand::Rng;

pub fn rev_wordban(token: String, channel: String, list: Vec<String>) {

    //println!("starting banlist");

    let mes = rev_read2(token.clone(), channel.clone());


    let mut delete = vec!["".to_string()];

    for mut x in 0..mes.len() {
   
        let content = ajson::get(&mes[x], &"content".to_string()).unwrap();
        for mut y in 0..list.len() {

            if content.to_string() == list[y] {
                delete.push(mes[x].clone())
            };
        };

    };

    //println!("delete {:#?}", delete);
    if delete == vec!["".to_string()] {
        return
    }else {
        println!("{} illegal words found", delete.len() -1);
        rev_mass_delete(token.clone(), channel.clone(), delete);
    };


}
pub fn rev_mass_delete(token: String, channel: String, messages: Vec<String>) {

    let token = "x-bot-token: ".to_owned() + &token;
    let target = "https://api.revolt.chat/channels/".to_owned() + &channel + "/messages/bulk";

    let mut mes = String::new();
    for x in 0..messages.len() -1 {
   
        let mut current = ajson::get(&messages[x + 1], "_id").unwrap();

        mes = mes + "\"" + &current.to_string() + "\", ";
    };
    mes.pop();
    mes.pop();
            
    let json = r#"

        {
  "ids": [
    "#.to_owned() + &mes + r#"
  ]
}"#;
     
    println!("{}", json);

    let curl = Command::new("curl")
        .args([
             "-sS", "-X", "DELETE", &target,
             "-H", &token,
             "-H", "Content-Type: application/json",
             "--data", &json
        ]).output().expect("failed to curl");

        
        let curl_out = from_utf8(&curl.stdout).unwrap().to_string();

        let curl_err = from_utf8(&curl.stderr).unwrap().to_string();

        println!("{}{}", curl_out, curl_err);

}



pub fn rev_del(token: String, channel: String, target: String)  {

    let api = "https://api.revolt.chat/channels/".to_owned() + &channel + "/messages/" + &target;
    let token = "x-bot-token: ".to_owned() + &token;

   let  _curl =  Command::new("curl")
        .args([
               "-sS", "-X", "DELETE", &api,
              "-H", &token,
        ])
        .output()
        .expect("failed to run");


}

pub fn permcheck(user: String, sudoers: Vec<String>) -> bool {

    for x in 0..sudoers.len() {

        if user == sudoers[x] {
            return true
        };
    };
    return false
}
fn rev_history(token: String, channel: String, mut numget: i32) -> (Vec<String>, Vec<String>){

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

    let curl = Command::new("curl")
        .args([
               "-sS", "-X", "GET", &api,
              "-H", &token,
              "-H", "Content-Type: application/json",
        ])
        .output()
        .expect("failed to run");


    let curl_out = from_utf8(&curl.stdout).unwrap().to_string();

    let list = ajson::get(&curl_out, "#").unwrap().to_string();

    let list2 = list.parse::<i32>().unwrap();



    if numget > list2 {

        println!("invalid input for rev_history\nrequested {} messages but only {} found\n", numget, list);
        //return (vec!["no".to_string()], vec!["no".to_string()])

        numget = list2;
    }

    let mut x = 0;

        for _x in 0..numget {
            let iter1 = &(_x.to_string() + ".content");
            let iter2 = &(_x.to_string() + ".author");

            n.content.push("<placeholder>".to_string());
            n.author.push("<placeholder>".to_string());

            n.content[x] = ajson::get(&curl_out, iter1).unwrap().to_string();
            n.author[x] = ajson::get(&curl_out, iter2).unwrap().to_string();

             x = x + 1;
        }
            return (n.content, n.author);

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


   let content = ajson::get(&send_out1_stdout, "0.content").unwrap().to_string();
   let author =  ajson::get(&send_out1_stdout, "0.author").unwrap().to_string();
   let id = ajson::get(&send_out1_stdout, "0._id").unwrap().to_string();

   return (content, author, id)


}

pub fn rev_read2(token: String, channel: String) -> Vec<String> {


    let api = "https://api.revolt.chat/channels/".to_owned() + &channel + "/messages";
    let token = "x-bot-token: ".to_owned() + &token;



    let curl = Command::new("curl")
        .args([
               "-sS", "-X", "GET", &api,
              "-H", &token,
              "-H", "Content-Type: application/json",
        ])
        .output()
        .expect("failed to run");


    let curl_out = from_utf8(&curl.stdout).unwrap().to_string();


   //let content = ajson::get(&send_out1_stdout, "0.content").unwrap().to_string();
   //let author =  ajson::get(&send_out1_stdout, "0.author").unwrap().to_string();
   //let id = ajson::get(&send_out1_stdout, "0._id").unwrap().to_string();

    let mut vec = vec![];
    
    let mut curl_len = ajson::get(&curl_out, "#").unwrap().to_string();
   
    let curl_len = curl_len.parse::<i32>().unwrap();

    let mut x = 0;
    
    for _x in 0..curl_len {

        vec.push("PLACEHOLDER".to_string());
        let temp = ajson::get(&curl_out, &x.to_string()).unwrap().to_string();
        vec[x] = temp;

        x = x + 1;
    }

    return vec

}



fn rev_search(token: String, channel: String, content: String, limit: i8) {

    let api = "https://api.revolt.chat/channels/".to_owned() + &channel + "/search";
    let token = "x-bot-token: ".to_owned() + &token;


    //  DO NOT LINT 
    let content_print = r#"{
  "query": ""#.to_owned() + &content + r#"",
  "limit": "# + &limit.to_string() + r#",
  "sort": "Latest",
  "include_users": true"# + r#"
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

    let _curl = Command::new("curl")
        .args([
               "-sS", "-X", "POST", &api,
              "-H", &token,
              "-H", &idem_print,
              "-H", "Content-Type: application/json",
              "--data", &content_print
        ])
        .output()
        .expect("failed to run");


    //let curl_out = from_utf8(&curl.stdout).unwrap().to_string();

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
     //let curlerr = from_utf8(&curl.stderr).unwrap().to_string();

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
use std::process::Command;
use std::str::from_utf8;
use rand::Rng;

pub fn rev_wordban(token: String, channel: String, list: Vec<String>) {

    //println!("starting banlist");

    let mes = rev_read2(token.clone(), channel.clone());


    let mut delete = vec!["".to_string()];

    for mut x in 0..mes.len() {
   
        let content = ajson::get(&mes[x], &"content".to_string()).unwrap();
        for mut y in 0..list.len() {

            if content.to_string() == list[y] {
                delete.push(mes[x].clone())
            };
        };

    };

    //println!("delete {:#?}", delete);
    if delete == vec!["".to_string()] {
        return
    }else {
        println!("{} illegal words found", delete.len() -1);
        rev_mass_delete(token.clone(), channel.clone(), delete);
    };


}
pub fn rev_mass_delete(token: String, channel: String, messages: Vec<String>) {

    let token = "x-bot-token: ".to_owned() + &token;
    let target = "https://api.revolt.chat/channels/".to_owned() + &channel + "/messages/bulk";

    let mut mes = String::new();
    for x in 0..messages.len() -1 {
   
        let mut current = ajson::get(&messages[x + 1], "_id").unwrap();

        mes = mes + "\"" + &current.to_string() + "\", ";
    };
    mes.pop();
    mes.pop();
            
    let json = r#"

        {
  "ids": [
    "#.to_owned() + &mes + r#"
  ]
}"#;
     
    println!("{}", json);

    let curl = Command::new("curl")
        .args([
             "-sS", "-X", "DELETE", &target,
             "-H", &token,
             "-H", "Content-Type: application/json",
             "--data", &json
        ]).output().expect("failed to curl");

        
        let curl_out = from_utf8(&curl.stdout).unwrap().to_string();

        let curl_err = from_utf8(&curl.stderr).unwrap().to_string();

        println!("{}{}", curl_out, curl_err);

}



pub fn rev_del(token: String, channel: String, target: String)  {

    let api = "https://api.revolt.chat/channels/".to_owned() + &channel + "/messages/" + &target;
    let token = "x-bot-token: ".to_owned() + &token;

   let  _curl =  Command::new("curl")
        .args([
               "-sS", "-X", "DELETE", &api,
              "-H", &token,
        ])
        .output()
        .expect("failed to run");


}

pub fn permcheck(user: String, sudoers: Vec<String>) -> bool {

    for x in 0..sudoers.len() {

        if user == sudoers[x] {
            return true
        };
    };
    return false
}
fn rev_history(token: String, channel: String, mut numget: i32) -> (Vec<String>, Vec<String>){

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

    let curl = Command::new("curl")
        .args([
               "-sS", "-X", "GET", &api,
              "-H", &token,
              "-H", "Content-Type: application/json",
        ])
        .output()
        .expect("failed to run");


    let curl_out = from_utf8(&curl.stdout).unwrap().to_string();

    let list = ajson::get(&curl_out, "#").unwrap().to_string();

    let list2 = list.parse::<i32>().unwrap();



    if numget > list2 {

        println!("invalid input for rev_history\nrequested {} messages but only {} found\n", numget, list);
        //return (vec!["no".to_string()], vec!["no".to_string()])

        numget = list2;
    }

    let mut x = 0;

        for _x in 0..numget {
            let iter1 = &(_x.to_string() + ".content");
            let iter2 = &(_x.to_string() + ".author");

            n.content.push("<placeholder>".to_string());
            n.author.push("<placeholder>".to_string());

            n.content[x] = ajson::get(&curl_out, iter1).unwrap().to_string();
            n.author[x] = ajson::get(&curl_out, iter2).unwrap().to_string();

             x = x + 1;
        }
            return (n.content, n.author);

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


   let content = ajson::get(&send_out1_stdout, "0.content").unwrap().to_string();
   let author =  ajson::get(&send_out1_stdout, "0.author").unwrap().to_string();
   let id = ajson::get(&send_out1_stdout, "0._id").unwrap().to_string();

   return (content, author, id)


}

pub fn rev_read2(token: String, channel: String) -> Vec<String> {


    let api = "https://api.revolt.chat/channels/".to_owned() + &channel + "/messages";
    let token = "x-bot-token: ".to_owned() + &token;



    let curl = Command::new("curl")
        .args([
               "-sS", "-X", "GET", &api,
              "-H", &token,
              "-H", "Content-Type: application/json",
        ])
        .output()
        .expect("failed to run");


    let curl_out = from_utf8(&curl.stdout).unwrap().to_string();


   //let content = ajson::get(&send_out1_stdout, "0.content").unwrap().to_string();
   //let author =  ajson::get(&send_out1_stdout, "0.author").unwrap().to_string();
   //let id = ajson::get(&send_out1_stdout, "0._id").unwrap().to_string();

    let mut vec = vec![];
    
    let mut curl_len = ajson::get(&curl_out, "#").unwrap().to_string();
   
    let curl_len = curl_len.parse::<i32>().unwrap();

    let mut x = 0;
    
    for _x in 0..curl_len {

        vec.push("PLACEHOLDER".to_string());
        let temp = ajson::get(&curl_out, &x.to_string()).unwrap().to_string();
        vec[x] = temp;

        x = x + 1;
    }

    return vec

}



fn rev_search(token: String, channel: String, content: String, limit: i8) {

    let api = "https://api.revolt.chat/channels/".to_owned() + &channel + "/search";
    let token = "x-bot-token: ".to_owned() + &token;


    //  DO NOT LINT 
    let content_print = r#"{
  "query": ""#.to_owned() + &content + r#"",
  "limit": "# + &limit.to_string() + r#",
  "sort": "Latest",
  "include_users": true"# + r#"
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

    let _curl = Command::new("curl")
        .args([
               "-sS", "-X", "POST", &api,
              "-H", &token,
              "-H", &idem_print,
              "-H", "Content-Type: application/json",
              "--data", &content_print
        ])
        .output()
        .expect("failed to run");


    //let curl_out = from_utf8(&curl.stdout).unwrap().to_string();

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
     //let curlerr = from_utf8(&curl.stderr).unwrap().to_string();

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
