/// # Handles HTTP errors and other methods
///
/// ### Web
///```
///use reywen::debug::Web;
///pub async fn http_example() {
///    if let Err(e) = reqwest::Client::new()
///        .get(format!("https://example.com"))
///        .send()
///        .await
///    {
///        Web::error(e, "example_function");
///    };
///}
///```
///The default functionality is to print the http error to stdout, this can be modified
///More debug features will be added but for now it's just HTTP
pub mod debug;

/// # Methods are implementations of the Revolt API
mod methods {
    pub mod channel;
    pub mod message;
    pub mod server;
    pub mod user;
}
/// Data structures (copied mainy from the Revolt backend) for API
pub mod structs {
    pub mod attachment;
    pub mod auth;
    pub mod channel;
    pub mod message;
    pub mod server;
    pub mod user;
}

/// An easy builder pattern for interacting with API
/// ## Defining Client
/// input message is type `Message`, and is given by
/// websocket
///
///
/// Auth is type `Auth` and needs to be imported as a json
/// or hardcoded in the binary
/// ```
///use reywen::client::Do;
///use reywen::structs::message::Message;
///use reywen::structs::auth::Auth;
///async fn client_example(auth: Auth, input_message: Message) {
///      let client = Do::new(&auth, &input_message);
///}
///```
/// ## Message Client
///
/// The input message can (and is intended to) be queried
/// through client. There are multiple methods for returning
/// both a `bool` and `String` for the types in `Message`
/// ```
///use reywen::client::Do;
///async fn message_process(client: Do) {
/// // bool
///if client.content_is("hello") {
///    println!("someone said hi!");
///}
/// // str
///else if client.content() == "world" {
///   println!("indeed")
///};
///
///
/// //The 'real' type for `Message.content` is `Option<String>`
/// //This is converted to `String` but will default to `String::new()` if there is no content
/// //It is important to keep this in mind when handling messages
///
/// //This schema is the same for *most* variables in User, however it is not feature complete as of `reywen.rs v0.1.5`
///
///let message_author = client.author();
///}
///```
///
/// As a temporary measure the fields of the Do struct are publically available
/// But this **will be depricated in future**, keep this in mind
///```
/// //The main problem with this is that additional cloning and borrowing is required
/// //Which is undesirable for those two are still not familiar with the borrow checker
///use reywen::client::Do;
///async fn replies_example(client: Do) {
///let replies: Option<Vec<String>> = client.input_message.replies.clone();
///}
///```
/// ## Polling with Client
/// As previously mentioned, variables can be defined with client from the input_message
/// Additionally, data can be defined through methods of Client which poll API
///```
///use reywen::client::Do;
///async fn user_example(client: Do) {
///let user = client.self_fetch().await.unwrap();
/// println!("nice badge :o {:?}", user.badges);
///}
///```
///
///
/// ## Sending to API
/// Client has several methods for sending messages to API, the two mains ones are `send` and `sender`
///```
///use reywen::structs::message::DataMessageSend;
///use reywen::client::Do;
///async fn send_example(client: Do) {
/// //send requires a payload, which can be fully typed
///let payload = DataMessageSend {
///     content: Some(String::from("hello")),
///     replies: None,
///     embeds: None,
///     masquerade: None,
///};
/// //or use default
///let mut payload = DataMessageSend::default();
///payload.content = Some(String::from("hello"));
///client.send(payload).await;
///
/// // sender only requires content, but content is mandatory
///client.sender("hello").await;
///}
///```
///Send provides greater control over the APi while sender is much easier to use
///In practise, sender will be used much more often.
pub mod client;

///## Websocket Configuration
///Websocket requires a few basic details for connection
///```
///use reywen::client::Do;
///use reywen::websocket::Websocket;
/// let ws = Websocket {
///     token: String::from("TOKEN"),
///     format: String::from("json"),
///     domain: String::from("ws.revolt.chat"),
///};
///```
///These can be imported as a json or hardcoded
///```
///use reywen::websocket::Websocket;
///  let file =
///        String::from_utf8(std::fs::read("config/reywen.json").expect("unable to find file config!"))
///            .expect("Failed to interpret byte array");
///
///let websocket = serde_json::from_str::<Websocket>(&file).expect("config is not type websocket");
///```
///## Establishing a connection
///This part of reywen is not to the same quality standard as I would like, websocket is difficult
///any help or suggestions would be apperciated
///
///### Example of Websocket
///```
/// // reywen moduals
///use reywen::{
///    client::Do,
///    structs::{auth::Auth, message::Message},
///    websocket::Websocket,
///};
///
/// // required for async websocket
///use futures_util::StreamExt;
///
///
///pub async fn example(websocket: Websocket, auth: Auth) {
///
/// // restart websocekt - always
/// loop {
///   Websocket::new(websocket.clone())
///         // generate establishes a connection based on credentials
///         // it also handles maintaining the websocket connnection
///         // there is no method to send messages to websocket
///         // but for revolt this is not needed
///         .generate()
///         .await
///             // for every message
///         .for_each(|message| async {
///
///             // if the message is valid
///             if let Ok(raw_message) = message {
///
///                 // and of type `Message`
///                 if let Ok(input_message) =
///                     serde_json::from_str::<Message>(&raw_message.into_text().unwrap())
///
///                 {
///                     // spawn a new task
///                     tokio::spawn(on_message(input_message, auth.clone()));
///                 }
///             }
///         })
///         .await;
///}}
/// // This function runs for every Message from websocket
///async fn on_message(input_message: Message, auth: Auth) {
///    // define a client
///    let client = Do::new(&auth, &input_message);
///
///   // Here you can add your logic! refer to client documentation for methods
///}
///```
pub mod websocket;
