// external
use mongodb::{options::ClientOptions, bson::doc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RMongo {
    pub username: String,
    pub password: String,
    pub ip: Option<String>,
    pub port: Option<String>,
    pub database: String
}

#[allow(dead_code)]
pub async fn mongo_db(mut mongo: RMongo) -> mongodb::Database {

    if mongo.ip.is_none() {
        mongo.ip = Some(String::from("localhost"))
    };
    if mongo.port.is_none() {
        mongo.port = Some(String::from("27017"))
    };

    let options = format!("mongodb://{}:{}@{}:{}",
                        mongo.username, mongo.password, mongo.ip.unwrap(), mongo.port.unwrap());

    let client = mongodb::Client::with_options(ClientOptions::parse(options).await.unwrap())
        .expect("could not connect to database");

    client.database(&mongo.database)
}

impl RMongo {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn username(mut self, username: &str) -> Self {
        self.username = String::from(username);
        self
    }
    #[allow(dead_code)]
    pub fn password(mut self, password: &str) -> Self {
        self.password = String::from(password);
        self
    }
    #[allow(dead_code)]
    pub fn ip(mut self, ip: &str) -> Self {
        self.ip = Some(String::from(ip));
        self
    }
    #[allow(dead_code)]
    pub fn port(mut self, port: &str) -> Self {
        self.port = Some(String::from(port));
        self
    }
    #[allow(dead_code)]
    pub fn database(mut self, db: &str) -> Self {
        self.database = String::from(db);
        self
    }

}

// example usage of mongodb
#[allow(dead_code)]
async fn example_usage() {

    // define your credentials
    let dbinfo = RMongo::new()
        .username("github")
        .password("hello")
        .database("test");

    // establish connection with mongodb
    let db = mongo_db(dbinfo).await;

    // from here, mongodb can be used normally
    let collection = db.collection::<String>("names");

    let _ = collection.find_one(doc!("name": "jeff"), None).await;

}




