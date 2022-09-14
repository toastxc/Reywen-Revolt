# Reywen 
General purpose bot for Revolt.chat using the RevX library


you will need to add values to src/main.rs for the bot to be functional 
https://developers.revolt.chat/api

# Features

- __sendas__
Sends masqurade messages with custom profile picture 
- __mc__
Quries minecraft servers, returns true for online and false for offline

- __wordban__
Auto deletes words found in the banlist

There are more features comming soon, for now I am focused on broadening the capabilities of the RevX library

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

Information
- [x] server
- [ ] channel
- [ ] user

General
- [x] web socket session
- [x] external conf
