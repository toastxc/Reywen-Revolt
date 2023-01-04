
const DURL:  &str = "https://autumn.revolt.chat/attachments/6bfy1Es-xWa9U6VzEPSw7DnbQPGUDK7LWrk4yRWHpV";

use urlencoding::encode;
use crate::{lib::{conf::Auth, message::{RMessage, Masquerade, RMessagePayload}}, rev_x::rev_send, fs::fs_str};
use reqwest::header::USER_AGENT;
use serde::{Serialize, Deserialize};
use serde_json::Value;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct E6Conf {
    pub enabled: bool,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Root {
    #[serde(rename = "posts")]
    pub posts: Vec<Post>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Post {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "file")]
    pub file: File,
    #[serde(rename = "preview")]
    pub preview: Preview,
    #[serde(rename = "sample")]
    pub sample: Sample,
    #[serde(rename = "score")]
    pub score: Score,
    #[serde(rename = "tags")]
    pub tags: Tags,
    #[serde(rename = "locked_tags")]
    pub locked_tags: Vec<Value>,
    #[serde(rename = "change_seq")]
    pub change_seq: i64,
    #[serde(rename = "flags")]
    pub flags: Flags,
    #[serde(rename = "rating")]
    pub rating: String,
    #[serde(rename = "fav_count")]
    pub fav_count: i64,
    #[serde(rename = "sources")]
    pub sources: Vec<String>,
    #[serde(rename = "pools")]
    pub pools: Vec<Value>,
    #[serde(rename = "relationships")]
    pub relationships: Relationships,
    #[serde(rename = "approver_id")]
    pub approver_id: Option<i64>,
    #[serde(rename = "uploader_id")]
    pub uploader_id: Option<i64>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "comment_count")]
    pub comment_count: i64,
    #[serde(rename = "is_favorited")]
    pub is_favorited: bool,
    #[serde(rename = "has_notes")]
    pub has_notes: bool,
    #[serde(rename = "duration")]
    pub duration: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct File {
    #[serde(rename = "width")]
    pub width: i64,
    #[serde(rename = "height")]
    pub height: i64,
    #[serde(rename = "ext")]
    pub ext: String,
    #[serde(rename = "size")]
    pub size: i64,
    #[serde(rename = "md5")]
    pub md5: String,
    #[serde(rename = "url")]
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Preview {
    #[serde(rename = "width")]
    pub width: i64,
    #[serde(rename = "height")]
    pub height: i64,
    #[serde(rename = "url")]
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Sample {
    #[serde(rename = "has")]
    pub has: bool,
    #[serde(rename = "height")]
    pub height: i64,
    #[serde(rename = "width")]
    pub width: i64,
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(rename = "alternates")]
    pub alternates: Alternates,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Alternates {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Score {
    #[serde(rename = "up")]
    pub up: i64,
    #[serde(rename = "down")]
    pub down: i64,
    #[serde(rename = "total")]
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Tags {
    #[serde(rename = "general")]
    pub general: Option<Vec<String>>,
    #[serde(rename = "species")]
    pub species: Option<Vec<String>>,
    #[serde(rename = "character")]
    pub character: Option<Vec<String>>,
    #[serde(rename = "copyright")]
    pub copyright: Option<Vec<String>>,
    #[serde(rename = "artist")]
    pub artist: Option<Vec<String>>,
    #[serde(rename = "invalid")]
    pub invalid: Option<Vec<String>>,
    #[serde(rename = "lore")]
    pub lore: Option<Vec<String>>,
    #[serde(rename = "meta")]
    pub meta: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Flags {
    #[serde(rename = "pending")]
    pub pending: bool,
    #[serde(rename = "flagged")]
    pub flagged: bool,
    #[serde(rename = "note_locked")]
    pub note_locked: bool,
    #[serde(rename = "status_locked")]
    pub status_locked: bool,
    #[serde(rename = "rating_locked")]
    pub rating_locked: bool,
    #[serde(rename = "comment_disabled")]
    pub comment_disabled: bool,
    #[serde(rename = "deleted")]
    pub deleted: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Relationships {
    #[serde(rename = "parent_id")]
    pub parent_id: Option<i64>,
    #[serde(rename = "has_children")]
    pub has_children: bool,
    #[serde(rename = "has_active_children")]
    pub has_active_children: bool,
    #[serde(rename = "children")]
    pub children: Option<Vec<i32>>,
}

pub async fn e6_main(auth: Auth, input_message: RMessage) {
     
     
     let conf: String = fs_str("config/e6.json").expect("failed to read config/e6.json\n{e}");

     let e6: E6Conf = serde_json::from_str(&conf)
            .expect("Failed to deser e6.json");
     
     // fail conditions
     if !e6.enabled {
         return
         
     }else if input_message.content.is_none() {
         return
     };
     let temp = input_message.content.unwrap();
     let convec: Vec<&str> = temp.split(' ').collect();
     
     if convec.len() < 3 {
         return
     }else if convec[0] != "?e" {
         return 
     };
     
     if ping_test(&e6.url).await {
         e6_send(&input_message.channel, &auth.token, "could not reach e6!!").await;
     }
     
     let var = match convec[1] as &str {
         "search" => e6_search(&convec, &e6.url).await,
         _ => return,
         
     };
     
     if var != String::new() {
          e6_send(&input_message.channel, &auth.token, &var).await;
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

 async fn e6_send(channel: &str, token: &str, content: &str) {
    let masq: Masquerade =  Masquerade {
        name: Some("E621".to_string()),
        avatar: Some(String::from("https://avatars.githubusercontent.com/u/105477506?s=200&v=4")),
        colour: None,
    };
    let payload = RMessagePayload {
        content: Some(content.to_string()),
        replies: None,
        masquerade: Some(masq),
        attachments: None,
        
    };
    
    rev_send(token, channel, payload).await;
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
            (3, _)                   => format!("**UwU**\n[]({})", img1),
            (4, 1)                   => format!("**UwU**\narg ignored, one result found\n[]({})", img1),
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
            return format!("**UwU**\n[]({})", img1);             
        };
        
        String::from("**Invalid request!**")
 }
 
