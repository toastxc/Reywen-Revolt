use crate::{
    MainConf, RMessage, rev_message_clean, send, sendas};

// main message engine 
pub async fn message_process(details: MainConf, message_in: RMessage) {

    if details.message.message_enabled == false {
        return
    };

    let content = message_in.content.clone();
    // validity test
    if content == None {
        return
    }else if message_in.author == details.auth.bot_id {
        return
    };
    let message = rev_message_clean(message_in).await;

    let content_vec =  content.as_ref().expect("failed to split vec").split(' ').collect::<Vec<&str>>();

    let mut content_min1 = String::new();

    for x in 0..content_vec.len() -1 {
        content_min1 += &format!("{} ", content_vec[x + 1])
    };


    match &content_vec[0] as &str {

        "?Mog" | "?mog"  => send(details.auth, message, ":01G7MT5B978E360NB6VWAS9SJ6:".to_string()).await,
        "?ver" | "?version" => send(details.auth, message, "**Version**\nReywen: `2`\nRevX: `2`".to_string()).await,
        "?echo" => send(details.auth, message, content_min1).await,
        "?sendas" => sendas(details.auth, message, content_vec).await,
        "?whydoyounothaveanyfeatures:trol:" => send(details.auth, message, "Reywen1 was an experiment to see if <@01FSRTTGJC1XJ6ZEQJMSX8Q96C> could make a bot for revolt with no libraries in rust\n Reywen2 was a test to see how much better they could make it with more experience in rust... if you want to make add features Reywen is not difficult to develop for \n".to_string()).await,
        _ => return
    };

}

