// dependancies 

use crate::{Auth, RMessage, lib::message::*};
use crate::fs_str;
use serde::{Serialize, Deserialize};
use crate::send;
use mongodb::{options::ClientOptions, bson::doc};
//use crate::rev_convert_reply;
use crate::rev_send;
use bson::Document;
//use mongodb::results::CollectionType::Collection;
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
pub async fn plural_main(a: Auth, m: RMessage) {

    
    let conf = fs_str("config/plural.json")
        .expect("failed to read config/plural.json\n{e}");

    let c: Plural = serde_json::from_str(&conf)
        .expect("Failed to deser plural.json");

        


    // if the config channel matches the channel of the message received AND 
    // if the plugin is enabled, send ID
    if c.enable == false {
        return
    
    }else if c.channel_only == true && c.channel != m.channel {
        return
    }; 

    let content_raw = match m.content {
        Some(_) => m.content.as_ref().unwrap(),
        None => return,
    };

    
    let content: Vec<&str> =  content_raw.split(' ').collect::<Vec<&str>>();

    if content[0] != "?p" {
           return
       }else if content.len() < 2 {
    
        return 
    }else if m.author == a.bot_id {
        return
    };

 

    

    match &content[1] as &str {

        "insert" => pl_insert(a, m.clone(), c, content).await,
        "send" => pl_send(a, m.clone(), content, c).await,
        "search" => cli_search(a, m.clone(), content[2], c).await,
        "rm" => pl_remove(a, m.clone(), content[2], c).await,
        //"generic" => pl_generic(a, m.clone(), content, c).await,
        _ => {},

    };

}


async fn cli_search(a: Auth, m: RMessage, i: &str, c: Plural)  {

    let res = pl_search(i, c).await;
    
    let strr = match res {
        Some(_) => "**Object found**",
        None => "**Object not found**",
    };

    send(a, m, strr.to_string()).await


}


async fn pl_remove(a: Auth, m: RMessage, i: &str, c: Plural)  {


    
    let name = i;
    println!("{name}");
    let param = format!("mongodb://{}:{}@{}:{}",
                        c.db_usrname, c.db_pswd, c.db_ip, c.db_port);

    let client_options = ClientOptions::parse(param).await.unwrap();

    let client = mongodb::Client::with_options(client_options);

    let client = match client {
        Ok(_) => client.unwrap(),
        Err(e) => {println!("failed:\n{e}"); return},
    };

    let db = client.database("test");

    let masks = db.collection::<Masquerade>("profiles");

    let userquery = masks.find_one(doc! { "name": name }, None).await;


    if userquery.is_ok() != true {
        send(a, m, "**Failed to get details**".to_string()).await;
        println!("WARN: pl_remove failed to connect");
    
    }else if userquery.unwrap().is_some() != true {
        send(a, m, "**No object found**".to_string()).await
    }else {
        let del_res = masks.delete_one(doc!{"name": name}, None ).await;
        send(a.clone(), m.clone(), "**Object found, deleting...**".to_string()).await;

        match del_res {
            Ok(_) => send(a, m, "**Successfully deleted**".to_string()).await,
            Err(e) => send(a, m, format!("**Error**\n```text\n{e}")).await,
        };
    };

}




async fn pl_search(i: &str, c: Plural) -> Option<Masquerade> {


    let name = i;
    println!("{name}");
    let param = format!("mongodb://{}:{}@{}:{}",
                        c.db_usrname, c.db_pswd, c.db_ip, c.db_port);

    let client_options = ClientOptions::parse(param).await.unwrap();

    let client = mongodb::Client::with_options(client_options);

    let client = match client {
        Ok(_) => client.unwrap(),
        Err(e) => {println!("failed:\n{e}"); return None},
    };

    let db = client.database("test");

    let masks = db.collection::<Masquerade>("profiles");

    let userquery = masks.find_one(doc! { "name": name }, None).await;

    match userquery {
        Ok(a) => return a,
        Err(e) => {println!("{e}"); return None},
    };
}

async fn pl_send(a: Auth, m: RMessage, i: Vec<&str>, c: Plural) {

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

        rev_send(a.clone(), m.clone(), payload),
        rev_del(a.clone(), m.clone()),
     );

    }else {

        send(a, m, "**Object  not found**".to_string()).await;

    };

}

async fn pl_insert(a: Auth, m: RMessage, c: Plural, i: Vec<&str>){

     let param = format!("mongodb://{}:{}@{}:{}",
                        c.db_usrname, c.db_pswd, c.db_ip, c.db_port);


      let client_options = ClientOptions::parse(param).await.unwrap();

      let client = mongodb::Client::with_options(client_options);

      let client = match client {
        Ok(_) => client.unwrap(),
        Err(e) => {println!("failed:\n{e}"); return},
    };

      let db = client.database("test");

      let collection = db.collection::<Document>("profiles");

      let mut data = vec![];

      let (name, avatar, colour) = (i[2], i[3], i[4]);
      let masks = doc! {
         "name": name,
         "avatar": avatar,
         "colour": colour,
      };

      data.push(masks);


      let userquery = collection.insert_many(data, None).await;

      if userquery.is_ok() != true {
          send(a, m, "**Failed to connect**".to_string()).await;
          println!("WARN: pl_insert failed to insert");

      }else {
          send(a, m, "**Object valid, inserting...**".to_string()).await;
      };

}
