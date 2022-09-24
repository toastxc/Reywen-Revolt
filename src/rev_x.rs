use std::process::Command;
use std::str::from_utf8;
use rand::Rng;
use ajson::*;


pub fn rev_server(token: String, channel: String) -> String {

    let url = format!("https://api.revolt.chat/channels/{channel}");
    let token = format!("x-bot-token: {token}");

    let curl = Command::new("curl")
        .args([

              "-sS", "-X", "GET", &url, 
              "-H", &token,
        ]).output().expect("failed to curl");

    let curl_out = from_utf8(&curl.stdout).unwrap().to_string();

    return ajson::get(&curl_out, "server").unwrap().to_string();

}

pub fn rev_kick(token: String, channel: String, mut content: String) {
    

    let server = rev_server(token.clone(), channel.clone());
    
    content.pop();
    content.remove(0);
    content.remove(0);
 
    
    println!("{content}");
    let url = format!("https://api.revolt.chat/servers/{server}/members/{content}"); 
    let token = format!("x-bot-token: {token}");


    let curl = Command::new("curl")
        .args([
              "-sS", "-X", "DELETE", &url,
              "-H", &token, 
        ]).output().expect("failed to run");


    let curl_out = from_utf8(&curl.stdout).unwrap().to_string();


    println!("{curl_out}");

}



pub fn purge(token: String, channel: String, num: String) {


    let raw = rev_read2(token.clone(), channel.clone());

    let mut returner: Vec<String> = vec![];

    let mut number = num.parse::<i32>();

    let mut err: bool = true;

    for x in 0..49 {
        if number == Ok(x) {
            err = false;
            number = Ok(number.unwrap() );

        }else {
        };
    };

    let mut y = 0;

    if err == false {

        for x in 0..number.clone().unwrap() {
        returner.push(ajson::get(&raw[y], "_id").unwrap().to_string());
        y = y + 1;
        };

        println!("{:?}", returner);

        rev_mass_delete(token, channel, returner)

    }else {
        rev_send(token, channel, "invalid number".to_string());
    }
    println!("{:?}", number);
}


pub fn wordban(token: String, channel: String, list: Vec<String>, content: String, id: String) {

    let mut delete = String::new();


        for mut y in 0..list.len() {

            if content == list[y] {
                delete = id.clone()
            };
        };


    //println!("delete {:#?}", delete);
    if delete == "" {
        return
    }else {
        println!("illegal words found: {}", content);
        rev_del(token.clone(), channel.clone(), delete.to_string());
        rev_send(token.clone(), channel.clone(), "no bad words!! grrr".to_string());

    };


}


pub fn rev_mass_delete(token: String, channel: String, messages: Vec<String>) {

    let token = format!("x-bot-token: {token}");
    let target = "https://api.revolt.chat/channels/".to_owned() + &channel + "/messages/bulk";

    let mut mes = String::new();
    for x in 0..messages.len()  {
   
        //let mut current = ajson::get(&messages[x], "_id").unwrap();
    
        let current = &messages[x];

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

    let token = format!("x-bot-token: {token}");

    let api = format!("https://api.revolt.chat/channels/{channel}/messages");

    // RNG
    let mut random = rand::thread_rng();
    let idem: i16 = random.gen();
    let idem_print = format!("Idempotency-Key: {idem}");
    

    let content_print = format!("{{\n\"content\":\"{content}\"\n}}");


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
