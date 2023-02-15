/// # Methods are implementations of the Revolt API
mod methods {
    pub mod bots;
    pub mod channel;
    pub mod member;
    pub mod message;
    pub mod relationships;
    pub mod server;
    pub mod user;
}
/// Data structures (copied mainy from the Revolt backend) for API
pub mod structs {
    pub mod attachment;
    pub mod auth;
    pub mod bots;
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
///
/// Additionally, Auth can be created from a token (you must declare if the token is for a bot or not)
/// ```
///use reywen::client::Do;
///use reywen::structs::message::Message;
///use reywen::structs::auth::Auth;
///async fn client_example(auth: Auth, input_message: Message) {
///
///      let auth = Auth::from_token("", true);
///      let client = Do::new(&auth, &input_message);
///
///}
///```
/// ## Input
///
/// The input message can be accessed with the method `input()`
/// for most fields in `input()` there are a `bool` and `String` based method
/// ```
///use reywen::client::Do;
///async fn message_process(client: Do) {
/// // bool
///if client.input().content_is("hello") {
///    println!("someone said hi!");
///}
/// // str
///else if client.input().content() == "world" {
///   println!("indeed")
///};
///
///
/// //The 'real' type for `Message.content` is `Option<String>`
/// //This is converted to `String` but will default to `String::new()` if there is no content
/// //It is important to keep this in mind when handling messages
///
///let message_author = client.input().author();
///}
///```

/// ## Message
/// ### Sending Messages
///```
///use reywen::client::Do;
///use reywen::structs::message::DataMessageSend;
///async fn message_example(client: Do) {
///  let message = client.message();
///
///  // send message without Message structure
///  message.sender("hello").await;
///
///  //'send' requires a payload, which can be fully typed
///
///  let payload = DataMessageSend::new().content("hewo");
///
///  message.send(payload).await;
///}
///```
/// ### Deleting messages
///
///```
///use reywen::client::Do;
///use reywen::structs::message::DataMessageSend;
///async fn message_example(client: Do) {
///  let message = client.message();
///
///  // send message without Message structure
///  message.sender("hello").await;
///
///  //'send' requires a payload, which can be fully typed
///
///  let payload = DataMessageSend::new().content("hewo");
///
///  message.send(payload).await;
///}
///```
/// ## Member
///
/// The member can be specified, but it will default to that of input_message
///
///```
///use reywen::client::Do;
///use reywen::structs::message::DataMessageSend;
///async fn member_example(client: Do) {
///
///   let server = client.server(None).await;
///
///   let member = server.member(None);
///
///   member.kick().await;
///
///}
///```
///
/// ## Channel
/// the channel method will default to the same channel as input_message unless declared
/// this example returns the channel details of the channel from input_message
///
/// Channel can be used to fetch, delete and edit the given channel
/// note: Message is its own structure
///```
///use reywen::client::Do;
///async fn channel_example(client: Do) {
///
///    let channel = client.channel(None);
///
///  println!("{:?}", channel.fetch().await);
///}
///```
///
/// ## Server
/// Like Channnel, the Server method has an optional input field.
/// ### warning: the option None has a chance of panicing if the message sent is not in a server
///```
///use reywen::client::Do;
///async fn server_example(client: Do) {
///
///  // Will painic if not for a server
///  let server = client.server(None).await;
///

///  server.leave().await;
///
///}
///```
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
///   websocket.clone()
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
