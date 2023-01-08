pub async fn example_main(auth: Auth, input_message: &RMessage) {
    // reywen is the main method for this bot,
    // it can be used to interact with the API both normally and with abstrated methods
    let client = Reywen::new(auth, input_message);

// crash condition checks for a list of undesireable
    // the second variable is an optional 'command prefix',
    if crash_condition(input_message, Some("?m")) {return};


    // vector of content
    let convec = convec(input_message);


    // matches all possilbe inputs and defines desired results as an str to be sent
    let payload = match convec[1] as &str {
        "?mog" => ":01G7MT5B978E360NB6VWAS9SJ6:",
        "VeiledProduct" => "no",
        "sussing" => "amogus",
        _ => "NoneOption%",
    };
    if payload != "NoneOption%" {
        client.clone().sender(payload).await;
    };

    // this is an example of send (not sender)
    // its lower level, giving you more control over reywen while
    // requiring more boilerplate.
    // construct a payload and then send it!

    let payload = RMessagePayload::new().content("world").reply_from(input_message);

    if convec[1] == String::from("hello!") {

        client.clone().send(payload).await;
    };
    // the available fields on RMessagePayload are the same as the API request requirements,
    // with additional fields for abstration



    // there are two main ways to use delete_msg, automated and command line
    // both of which would most often be used for moderation
    // there are alternate uses but they will not be covered here

    // CLI
    if convec.len() > 2 {
        if convec[1] == "?del" {
            client.clone().delete_msg(convec[2]).await;
        };
    };

    // automated
    // this example is a banned word list
    let wordlist = vec!["badword", "anotherbadword!!", "illegalwords"];

    for x in wordlist.iter() {
        if convec.contains(x) {
            client.clone().delete_msg(&input_message._id).await;

        };
    };

    // kicking a user can simply be done by specifying the user ID
    //client.member_kick("USERID")
    // use this method with care, while i can't stop anyone i'd rather that reywen
    // is not used for spamming or abuse on revolt

    // simutanious actions
    // for network applications such as reywen multithreading is not needed
    // instead tasks can run at the same time on the same thread with tokio
    if &convec[1] as &str == "?async" {
        tokio::join!(
                client.clone().sender("hey!"),
                client.delete_msg(&input_message._id),
                );
    };
    // notice the lack of await - the future is handled by tokio join
    // this tool is restricted to revolt's API rate limit


    // there are many other methods and more comming soon! but this is enough of an example
    // to get started, enjoy ^^ - Toast

}

