// external
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use urlencoding::encode;

use crate::{
    delta::{
        fs::fs_to_str,
        lreywen::{convec, crash_condition, lte},
        oop::Reywen,
    },
    quark::delta::{auth::Auth, message::RMessage},
};

// internal

const DURL: &str =
    "https://autumn.revolt.chat/attachments/6bfy1Es-xWa9U6VzEPSw7DnbQPGUDK7LWrk4yRWHpV";
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct E6Conf {
    pub enabled: bool,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Poster {
    #[serde(skip_serializing_if = "Option::is_none", rename = "posts")]
    post: Option<Vec<Post>>,
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
pub struct Alternates {}

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
    let conf: String = fs_to_str("config/e6.json").expect("failed to read config/e6.json\n{e}");

    let e6: E6Conf = serde_json::from_str(&conf).expect("Failed to deser e6.json");

    if crash_condition(input_message, Some("?e")) {
        return;
    };

    let convec = convec(input_message);

    let client = Reywen::new(auth.clone(), input_message);

    // determines if e6 is allive
    if ping_test(&e6.url).await {
        client
            .clone()
            .sender(&format!("**Could not reach {}", e6.url))
            .await;
    };

    let var = match convec[1] as &str {
        "search" => e6_search(&convec, &e6.url).await,
        "help" => String::from("**Hewo!**\n`?e search <>` to search"),
        _ => return,
    };
    client.sender(&var).await;
}

async fn ping_test(url: &str) -> bool {
    let client: std::result::Result<reqwest::Response, reqwest::Error> =
        reqwest::Client::new().get(url).send().await;

    if client.is_ok() {
        return false;
    };
    true
}

async fn e6_search(convec: &[&str], url: &str) -> String {
    // https://e926.net/posts?tags=fox&limit=1&page=2
    // ?e search fox 2

    // query payload url - tags - page number
    let query = &format!(
        "{url}/posts.json?tags={}&limit=1&page={}",
        encode(convec[2]),
        numcheck(convec)
    );

    // http request
    let http: std::result::Result<reqwest::Response, reqwest::Error> = reqwest::Client::new()
        .get(query)
        // user agent used with permission
        .header(USER_AGENT, "libsixgrid/1.1.1")
        .send()
        .await;

    if http.is_err() {
        return String::new();
    };

    let http_payload = http.unwrap().text().await.unwrap();

    if http_payload.is_empty() {
        return String::new();
    };

    let res: Poster = serde_json::from_str(&http_payload).expect("failed to interpret E6 data");

    if res.post.is_none() {
        return String::from("**No Results!**");
    };
    let res = res.post.expect("Failed to get resuls");

    let image: String = match &res[0].file.url {
        None => DURL.to_string(),
        Some(a) => a.to_string(),
    };

    format!("**UwU**\n{}", lte(&image))
}

fn numcheck(convec: &[&str]) -> String {
    if convec.len() < 4 {
        return 1.to_string();
    };

    let maybe_number = convec[3].parse::<usize>();

    match maybe_number {
        Err(_) => 1,
        Ok(a) => {
            if a >= 750 {
                1
            } else {
                a
            }
        }
    }
    .to_string()
}
