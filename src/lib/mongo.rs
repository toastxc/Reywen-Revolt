// external
use mongodb::{options::ClientOptions, bson::doc};
use bson::Document;
#[derive(Debug, Default)]
pub struct RMongo {
    pub username: String,
    pub password: String,
    pub ip: Option<String>,
    pub port: Option<String>,
    pub database: String
}


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
    pub fn new() -> Self {
        Self::default()
    }
    pub fn username(mut self, username: &str) -> Self {
        self.username = String::from(username);
        self
    }
    pub fn password(mut self, password: &str) -> Self {
        self.password = String::from(password);
        self
    }
    pub fn ip(mut self, ip: &str) -> Self {
        self.ip = Some(String::from(ip));
        self
    }
    pub fn port(mut self, port: &str) -> Self {
        self.ip = Some(String::from(port));
        self
    }
    pub fn database(mut self, db: &str) -> Self {
        self.password = String::from(db);
        self
    }

}

async fn womp() {

    let womp = RMongo::new()
        .username("helo")
        .password("i forgor")
        .database("the mongo one");


    let database = mongo_db(womp).await;

}

