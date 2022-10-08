use crate::{MainConf, RMessage};
use std::process::Command;
pub async fn shell_main(details: MainConf, message: RMessage) {

    if details.shell.enabled == false {
        return
    }else if message.content == None {
        return
   };

    let (auth, shell, soc) = (details.auth, details.shell.clone(), details.shell.shell_channel.clone());

    let content_vec =  message.content.as_ref().expect("failed to split vec").split(' ').collect::<Vec<&str>>();

    if content_vec.len() <= 1 {
        return
    };

    let mut content_min1 = String::new();

    for x in 0..content_vec.len() -1 {
        content_min1 += &format!("{} ", content_vec[x + 1])
    };

    let user = message.author;

    let mut sudoer = false;
    for x in 0..auth.sudoers.len() {
        if user == auth.sudoers[x] {
            sudoer = true;
            break
        };
    };

    if soc.enabled == true && soc.channel == message.channel {
        if shell.whitelist_sudoers == true && sudoer == true {
           if content_vec[0] == "?/" {

               bash_exec(content_vec).await;
               
               
            };
        };
    };
}

pub async fn bash_exec(input: Vec<&str>) {

    println!("WARN: bashexec");

    let mut newmer = "";

    let mut com = Command::new(input[1]);

    for x in 0..input.len() -2 {
        com.arg(input[x+2]);
    };


    let stdout = com.output().unwrap().stdout;
    let stderr = com.output().unwrap().stderr;


    //let (stdout, stderr) = (com.stderr.clone(), com.stderr.clone());

    let out = format!("{}{}", String::from_utf8_lossy(&stdout), String::from_utf8_lossy(&stderr));

    if out.chars().count() <= 2000 {
        println!("{out}");
    }else {


        let out_vec = out.split('\n').collect::<Vec<&str>>();

        for x in 0..out_vec.len() {
            println!("{}", out_vec[x])
        };
    };

    //let out_vec = out.split('\n').collect::<Vec<&str>>();


    //println!("{}", out_vec.len());

    //for x in 0..out_vec.len()
    
 //   println!("{:#?}", com);




}
