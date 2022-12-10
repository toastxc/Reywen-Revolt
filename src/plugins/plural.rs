use crate::{Auth, RMessage, lib::message::*};
use crate::fs_str;
use serde::{Serialize, Deserialize};
use crate::send;
use mongodb::{options::ClientOptions, bson::doc};
use crate::rev_send;
use bson::Document;
use crate::rev_del;

// config struct
// this optional struct adds configurable paramaters that are hot changeable, config files are
// jsons and usually stored in config/
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Plural {
    pub enable: bool,
    pub channel_only: bool,
    pub channel: String,
    pub db_usrname: String,
    pub db_pswd: String,
    pub db_ip: String,
    pub db_port: String,

}

// plugin main is responsible for getting details and activiating functions based on conditions
pub async fn plural_main(auth: Auth, message: RMessage) {

    let conf = fs_str("config/plural.json")
        .expect("failed to read config/plural.json\n{e}");

    let plural: Plural = serde_json::from_str(&conf)
        .expect("Failed to deser plural.json");


    // if the config channel matches the channel of the message received AND 
    // if the plugin is enabled, send ID
    if plural.enable == false {
        return
    
    }else if plural.channel_only == true && plural.channel != message.channel {
        return
    }; 

    let content_raw = match message.content {
        Some(ref a) => a,
        None => return,
    };

    let content: Vec<&str> =  content_raw.split(' ').collect::<Vec<&str>>();

    if content[0] != "?p" {
        return
    
    }else if content.len() < 3 {
        send(auth, message, "**Reywen Masq**
             `search <name>`: Search for an entry in ReywenDB
             `insert <name> <avatar-url> <color>`: create a new entry
             `query <name>`: searches for entry and provides details
             `send <name> <content>`: sends message as a given entry
             `rm <name>`: removes entry".to_string()).await;
        return
    
    }else if message.author == auth.bot_id {
        return
    };

    match &content[1] as &str {

        "insert" => pl_insert(auth, message.clone(), plural, content).await,
        "send" => pl_send(auth, message.clone(), content, plural).await,
        "search" => cli_search(auth, message.clone(), content[2], plural).await,
        "rm" => pl_remove(auth, message.clone(), content[2], plural).await,
        "query" => cli_query(auth, message.clone(), content[2], plural).await,
        _ => {},
    };

}
async fn cli_query(auth: Auth, message: RMessage, content: &str, plural: Plural) {

    let search = pl_search(content, plural).await;

    match search {
        Some(a) => {
            let masq_data = format!("```json\n\"name:\" \"{}\"\n \"avatar\" \"{}\"\n\"colour:\" \"{}\"", 
                                    a.name.unwrap(), a.avatar.unwrap(), a.colour.unwrap());
            send(auth, message, masq_data).await;
        },
        None  => send(auth, message, "**Object not found**".to_string()).await,
    };
}

async fn cli_search(auth: Auth, message: RMessage, content: &str, plural: Plural)  {

    let res = pl_search(content, plural).await;
    
    let strr = match res {
        Some(_) => "**Object found**",
        None => "**Object not found**",
    };

    send(auth, message, strr.to_string()).await
}
async fn pl_remove(auth: Auth, message: RMessage, content: &str, plural: Plural)  {

    let param = format!("mongodb://{}:{}@{}:{}",
                        plural.db_usrname, plural.db_pswd, plural.db_ip, plural.db_port);

    let client = mongodb::Client::with_options(ClientOptions::parse(param).await.unwrap());

    let client = match client {
        Ok(_) => client.unwrap(),
        Err(e) => {println!("failed:\n{e}"); return},
    };

    let masks = client.database("test").collection::<Masquerade>("profiles");

    let userquery = masks.find_one(doc! { "name": content }, None).await;

    if userquery.is_ok() != true {
        send(auth, message, "**Failed to get details**".to_string()).await;
        println!("WARN: pl_remove failed to connect");
    
    }else if userquery.unwrap().is_some() != true {
        send(auth, message, "**No object found**".to_string()).await
    }else {
        let del_res = masks.delete_one(doc!{"name": content}, None ).await;
        send(auth.clone(), message.clone(), "**Object found, deleting...**".to_string()).await;

        match del_res {
            Ok(_) => send(auth, message, "**Successfully deleted**".to_string()).await,
            Err(e) => send(auth, message, format!("**Error**\n```text\n{e}")).await,
        };
    };
}

async fn pl_search(content: &str, plural: Plural) -> Option<Masquerade> {

    let param = format!("mongodb://{}:{}@{}:{}",
                        plural.db_usrname, plural.db_pswd, plural.db_ip, plural.db_port);

    let client = mongodb::Client::with_options(ClientOptions::parse(param).await.unwrap());

     
    let client = match client {
        Ok(a) => a,
        Err(e) => panic!("MONGODB_FAILED_SEARCH:\n{e}"),
    };

    let db = client.database("test");

    let masks = db.collection::<Masquerade>("profiles");

    let userquery = masks.find_one(doc! { "name": content }, None).await;

    match userquery {
        Ok(a) => return a,
        Err(e) => {println!("{e}"); return None},
    };
}

async fn pl_send(auth: Auth, message: RMessage, i: Vec<&str>, c: Plural) {

    let profile = pl_search(i[2], c).await;

    if profile != None {

    let mut content = String::new();
        
    for x in 0..i.len() -3 {
        content = format!("{content} {}", i[x + 3]);
    };
    
    let payload = RMessagePayload {
        content: Some(content),
        attachments: None,
        replies: None,
        masquerade: Some(profile.unwrap()),
    };

    
    tokio::join!(

        rev_send(auth.clone(), message.clone(), payload),
        rev_del(auth, message),
        );

    }else {
        
        send(auth, message, "**Object  not found**".to_string()).await;
    };
}

async fn pl_insert(auth: Auth, message: RMessage, plural: Plural, content: Vec<&str>){

     let param = format!("mongodb://{}:{}@{}:{}",
                        plural.db_usrname, plural.db_pswd, plural.db_ip, plural.db_port);

     let client = mongodb::Client::with_options(ClientOptions::parse(param).await.unwrap());

         
     let client = match client {
        Ok(a) => a,
        Err(e) => {println!("MONGODB_insert:\n{e}"); return},
    
     };
      
     let collection = client.database("test").collection::<Document>("profiles");
      
     let data: Vec<Document> =
         vec![

         doc! {
         "name": content[2],
         "avatar": content[3],
         "colour": content[4],
     }];

     let userquery = collection.insert_many(data, None).await;

      if userquery.is_ok() != true {
          send(auth, message, "**Failed to connect**".to_string()).await;
          println!("WARN: pl_insert failed to insert");

      }else {
          send(auth, message, "**Object valid, inserting...**".to_string()).await;
      };
}
