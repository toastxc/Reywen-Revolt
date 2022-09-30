# Reywen 
A meme bot for Revolt.chat using the RevX2 library

RevX2 can be used to construct your own bot!

Currently working on a full redesign fork of this bot and it's library

# Example use

## Old method

```rust

if content == "?hello" {

  rev_send(data.token.clone(), channel.clone(), "hello user".to_string());

}
```
## New method
```rust
 match &content as &str {

        "?hello" => rev_send(data, message, format!("hello <@{}>", reply).to_string()).await,
        _ => return
    };

```

While the difference in syntax is minimal, there are many benefits of RevX2 over RevX1, such as
- async
- error handling
- serde_json (insteaed of ajson)
Features specific to Messages
- masqurade (properly)
- reply
- mentions

Using `match` instead of `else if` is more compact, and allows for more complex functions


For further information goto 
https://developers.revolt.chat/api
