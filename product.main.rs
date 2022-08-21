use core::str::from_utf8;
extern crate ajson;
use std::process::Command;
use rand::Rng;

#[derive(Debug, Clone)]
struct Data {

    token: String,
    channel: String,
    content: String,
}



fn main() {

    let data = Data {
    token: "".to_string(),
    channel: "".to_string(),
    content: "".to_string(),
    };




    //rev_read()
    // in: token, channel
    // out: latest message, author

    // rev_send()
    // in: token, channel, content (message)
    // out: <none>
    
    // rev_search()
    // in: token, channel, query
    // out: <none>
    // not complete
   
    // rev_history
    // in: token, channel, num_get
    // out: content, author


    


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



fn rev_read(token: String, channel: String) -> (String, String){


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


   return (content, author)
   

}


fn rev_search(token: String, channel: String, content: String, limit: i8) {

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



fn rev_send(token: String, channel: String, content: String) {


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

   // println!("REVOLT:\nster:\n{}\nstd:\n{}", send_out1_ster, send_out1_std);
        
}
