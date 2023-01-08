// external
use urlencoding::encode;
use reqwest::header::USER_AGENT;
use serde::{Serialize, Deserialize};
use serde_json::Value;

// internal
use crate::{lib::{fs::fs_to_str, lreywen::{crash_condition, convec, lte}, oop::Reywen}, structs::{auth::Auth, message::{RMessage, RMessagePayload, Masquerade}}};

const DURL:  &str = "https://autumn.revolt.chat/attachments/6bfy1Es-xWa9U6VzEPSw7DnbQPGUDK7LWrk4yRWHpV";
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct E6Conf {
    pub enabled: bool,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    #[serde(rename = "posts")]
    pub posts: Vec<Post>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub created_at: String,
    pub updated_at: String,
    pub file: File,
    pub preview: Preview,
    pub sample: Sample,
    pub score: Score,
    pub tags: Tags,
    pub locked_tags: Vec<Value>,
    pub change_seq: i64,
    pub flags: Flags,
    pub rating: String,
    pub fav_count: i64,
    pub sources: Vec<String>,
    pub pools: Vec<Value>,
    pub relationships: Relationships,
    pub approver_id: Option<i64>,
    pub uploader_id: Option<i64>,
    pub description: String,
    pub comment_count: i64,
    pub is_favorited: bool,
    pub has_notes: bool,
    pub duration: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub width: i64,
    pub height: i64,
    pub ext: String,
    pub size: i64,
    pub md5: String,
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Preview {
    pub width: i64,
    pub height: i64,
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sample {
    pub has: bool,
    pub height: i64,
    pub width: i64,
    pub url: Option<String>,
    pub alternates: Alternates,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Alternates {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Score {
    pub up: i64,
    pub down: i64,
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tags {
    pub general: Option<Vec<String>>,
    pub species: Option<Vec<String>>,
    pub character: Option<Vec<String>>,
    pub copyright: Option<Vec<String>>,
    pub artist: Option<Vec<String>>,
    pub invalid: Option<Vec<String>>,
    pub lore: Option<Vec<String>>,
    pub meta: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Flags {
    pub pending: bool,
    pub flagged: bool,
    pub note_locked: bool,
    pub status_locked: bool,
    pub rating_locked: bool,
    pub comment_disabled: bool,
    pub deleted: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Relationships {
    pub parent_id: Option<i64>,
    pub has_children: bool,
    pub has_active_children: bool,
    pub children: Option<Vec<i32>>,
}

pub async fn e6_main(auth: Auth, input_message: &RMessage) {
     
     // import config
     let conf: String = fs_to_str("config/e6.json")
        .expect("failed to read config/e6.json\n{e}");

     let e6: E6Conf = serde_json::from_str(&conf)
            .expect("Failed to deser e6.json");
     

     let masq = Masquerade::new()
        .name("E621")
        .avatar("https://avatars.githubusercontent.com/u/105477506?s=200&v=4");


     let payload = RMessagePayload::new()
        .masquerade(masq);
     
    if crash_condition(input_message, Some("?e")) {return};

    let convec = convec(input_message);

    let client = Reywen::new(auth.clone(), input_message);

    // determines if e6 is allive
     if ping_test(&e6.url).await {
         client.clone().send(payload.clone().content(&format!("**Could not reach {}", e6.url))).await;
     };
     
     let var = match convec[1] as &str {
         "search" => e6_search(&convec, &e6.url).await,
         "help"   => String::from("**Hewo!**\n`?e search <>` to search"),
         _ => return,
     };
     
     if var != String::new() {
          client.send(payload.content(&var)).await;
     };
}


async fn ping_test(url: &str) -> bool {
    
    let client: std::result::Result<reqwest::Response, reqwest::Error> =
        reqwest::Client::new() 
        .get(url)
        .send().await;
 
    if client.is_ok() {
        return false
    };
    true

}


async fn e6_search(convec: &Vec<&str>,  url: &str) -> String {
      
      let query = &format!("{url}/posts.json?tags={}", encode(convec[2])).to_string();
      
      let client: std::result::Result<reqwest::Response, reqwest::Error> =
        reqwest::Client::new() 
        .get(query)
        .header(USER_AGENT, "libsixgrid/1.1.1")
        .send().await;
        
        if client.is_err() { return String::new() };
      
        let payload = client.unwrap().text().await.unwrap();
            
        let res: Root = serde_json::from_str(&payload)
            .expect("failed to interpret E6 data");
            
        if res.posts.is_empty() {
            return "**No results!**".to_string();
        };
        let img1: String = match &res.posts[0].file.url {
            None => DURL.to_string(),
            Some(a) => a.to_string()
        };
        
        match (convec.len(), res.posts.len()) {
            // invalid
            (0, _) | (1, _) | (2, _) => "**Invalid query!**".to_string(),
            (_, 0)                   => "**No results!**".to_string(),
            // first result
            (3, _)                   => format!("**UwU**\n{}", lte(&img1)),
            (4, 1)                   => format!("**UwU**\narg ignored, one result found\n{}", lte(&img1)),
            // other result
            (4, _)                   => querycheck(convec, res),
            _ => "womp".to_string(),
        }   
  }
  
 
 fn querycheck(convec: &[&str], res: Root) -> String {
     
        let number = convec[3].parse::<u32>();
        if  number.is_ok() && res.posts.len() >= number.clone().unwrap() as usize {
             
            let img1: String = match &res.posts[number.unwrap() as usize].file.url {
                None => DURL.to_string(),
                Some(a) => a.to_string()
            };
            return format!("**UwU**\n{}", lte(&img1));
        };
        String::from("**Invalid request!**")
 }
