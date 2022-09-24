# Reywen 
A meme bot for Revolt.chat using the RevX library

RevX can be used to construct your own bot!

# Example use

```rust


if content == "?hello" {
                        rev_send(data.token.clone(), channel.clone(), "world!".to_string());

}else if content == "?ping" {
                        rev_send(data.token.clone(), channel.clone(), "Pong!!".to_string());
                    };
}

```   
For further information goto 
https://developers.revolt.chat/api


# API functions

Messages
- [x] sending messages
- [x] reading messages
- [x] reading chat history
- [x] delete messages
- [x] mass deleting messages
- [X] masqurade

Moderation
- [x] kicking users
- [ ] banning users
- [x] mass deleting banned words (100 per second)
- [x] word blacklist
- [x] user sudo check 

Information
- [x] server
- [ ] channel
- [ ] user

General
- [x] web socket session
- [x] external conf
