use crate::{MainConf, RMessage};

pub async fn shell_main(details: MainConf, message: RMessage) {

    if details.shell.enabled == false {
        return
    }else if message.content == None {
        return
   };

    let auth = details.auth;
    let shell = details.shell;
    let soc = shell.shell_channel;


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
           // if message.content.expect("this should be impossible").chars().nth(0) == "?/" {

            println!("{:?}", message.content.expect("this should be impossible").chars().nth(0).unwrap());
            println!("exec!!")
            //};
        };
    };
    /*

    println!("{:?}", details.shell);

    println!("{:?}  {:?}", details.shell.shell_only_channel.soc_channel, message.channel);
*/
    
}
