// internal
use crate::{structs::{message::{RMessage, Masquerade, RMessagePayload}, auth::Auth}, lib::{fs::fs_to_str, lreywen::{convec, crash_condition}, mongo::{RMongo, mongo_db}, oop::Reywen}};

// external
use serde::{Serialize, Deserialize};
use mongodb::bson::doc;

// config struct
// this optional struct adds configurable paramaters that are hot changeable, config files are
// jsons and usually stored in config/
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Plural {
    pub enabled: bool,
    pub channel_only: bool,
    pub channel: String,
}

// plugin main is responsible for getting details and activiating functions based on conditions
pub async fn plural_main(auth: Auth, input_message: &RMessage) {

    let conf = fs_to_str("config/plural.json")
        .expect("failed to read config/plural.json\n{e}");

    let plural: Plural = serde_json::from_str(&conf)
        .expect("Failed to deser plural.json");

    let mongo: RMongo = serde_json::from_str(&conf)
        .expect("Failed to deser plural.json");


    // if the config channel matches the channel of the message received AND 
    // if the plugin is enabled, send ID
    if !plural.enabled {
        return
    };
    if plural.channel_only && plural.channel != input_message.channel {
        return
    };

    let convec = convec(input_message);

    if crash_condition(input_message, Some("?p")) { return};

    // additional crash condition
    if convec.len() < 3 {return};

    let dbinfo = RMongo::new()
        .username(&mongo.username)
        .password(&mongo.password)
        .database(&mongo.database);

    let db = mongo_db(dbinfo).await;

    let client = Reywen::new(auth, input_message);


    match convec[1] as &str {
       "search" => {
           if pl_search(convec[2], db).await.is_none() {client.sender("**Profile could not be found!**").await;
           }else { client.sender("**Profile found!**").await;};},

        "rm" => {
           pl_remove(client.clone(), db, input_message).await;
       },
        "insert" => {
           pl_insert(client.clone(), db, input_message).await;
       },

        "send" => {
           pl_send(client.clone(), db, input_message).await;
       },
        "query" => {
           pl_query(input_message, db, client).await;
       },
        _ => {}
    };
}


async fn pl_search(query: &str, db: mongodb::Database) -> Option<Masquerade> {

    db
    .collection::<Masquerade>("profiles")
    .find_one(doc! { "name": query }, None).await.unwrap()

}

async fn pl_remove(client: Reywen, db: mongodb::Database, input_message: &RMessage) {

    let convec = convec(input_message);

    let collection = db.collection::<Masquerade>("profiles");

    let userquery = collection.find_one(doc! { "name": convec[2] }, None).await;

    if userquery.is_err() {
        client.sender("**Failed to connect to mongodb**").await;

    }else if userquery.unwrap().is_none() {
        client.sender("**No results found!**").await;
    }else {
        let del_res = collection.delete_one(doc!{"name": convec[2]}, None ).await;
        client.clone().sender("**Profile found, deleting...**").await;

        let str = match del_res {
            Ok(_) => String::from("**Successfully deleted**"),
            Err(e) => format!("**Error**\n```text\n{e}"),
        };
        client.sender(&str).await;
    };
}

async fn pl_send(client: Reywen, db: mongodb::Database, input_message: &RMessage) {


        let convec: Vec<&str> = convec(input_message);

    // ?p send <>
    let profile = pl_search(convec[2], db).await;

    if profile.is_none() {
        client.sender("**Invalid profile! (we couldn't find it pwp**").await;
        return
    };
    let profile = profile.unwrap();

    // turn the query into a sendable string
    let mut message = convec;
    message.remove(0);
    message.remove(0);
    message.remove(0);
    let new_message: String = message.iter().map(|i| i.to_string() + " ").collect();


    let mut payload = RMessagePayload::new()
        .masquerade(profile)
        .content(&new_message);

    // optional fields
    if input_message.replies.is_some() {
        payload = payload.reply_from(input_message);
    };

    tokio::join!(
            client.clone().send(payload),
            client.delete_msg(&input_message._id),
    );
}

async fn pl_insert(client: Reywen, db: mongodb::Database, input_message: &RMessage ) {


    let collection = db.collection::<Masquerade>("profiles");

    let convec = convec(input_message);

    if pl_search(convec[2], db).await.is_some() {
        client.clone().sender("**This profile already exists! try another name**").await;
        return
    };

    // CLI schema out of order ?p insert FLoofy --colour red --avatar img.jpg
    // no matter what there is always name

    let mut masq = Masquerade::new().name(convec[2]);

    // validity check and optional insertion
    for x in 0..convec.len()  - 1{
        // colour
        if convec[x] == "--colour" && convec[x + 1].chars().count() < 10 {
            masq = masq.colour(convec[x + 1]);
        };
        // avatar
        if convec[x] == "--avatar" && convec[x +1].chars().count() < 100 {
            masq = masq.avatar(convec[x + 1]);
        };
    };

    let userquery = collection.insert_one(masq, None).await;

    if userquery.is_err() {
        client.sender("**Failed to connect**").await;

    }else {
        client.sender("**Valid profile! adding to collection**").await;
    };
}

async fn pl_query(input_message: &RMessage, db: mongodb::Database, client: Reywen) {
    let convec = convec(input_message);
    // ?p query somethign
    let userquery = pl_search(convec[2], db).await;

    if userquery.is_none() {

        client.sender("**Could not find profile!**").await;
        return
    };
    let userquery = userquery.unwrap();

    let mut str = format!("```json\n{{\n\"name\": \"{}\"", userquery.name.unwrap());

    if userquery.avatar.is_some() {
        str += &format!(",\n\"avatar\": \"{}\"", userquery.avatar.unwrap());
    };
    if userquery.colour.is_some() {
        str += &format!(",\n\"colour\": \"{}\"", userquery.colour.unwrap());
    };

    str += "\n}\n```\n";
    client.sender(&str).await;
}