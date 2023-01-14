# Reywen - A Rust Bot and Framework for Revolt.chat
Reywen is a bot and a framework for building bots on the Revolt.chat platform, written in Rust. It provides simple abstraction libraries for easily implementing features, while allowing for more direct interaction with the Revolt APi and Websocket.

## Features
- Easy to use and well-documented API
- Functionality for most Revolt API methods
- Simple to use abstraction libraries for DX
- Support for the tokio async syntax
- Open-source and actively maintained
- Blazingly fast (duh)
- integeration and abstractions for MongoDB

## Things i won't add
- SQL support
- Discord integration
- Uploading images or other attachments


## Getting Started (for developers)

To develop for Reywen, you will need to have Rust installed on your machine. You can install rust by following the instructions on the [official website](https://www.rust-lang.org/learn/get-started)
```bash
git clone https://github.com/toastxc/Reywen-Revolt.git
cd Reywen-Revolt
```
Once you have your bot set up, you can start implementing your bot's functionality by editing the `src/plugins/message.rs` file. You will also need to set up your bot on Revolt.chat and get an API key, which you can use to authenticate your bot when connecting to the platform.

## Documentation
Reywen functions, abstractions and features
[can be found at the wiki](https://github.com/toastxc/Reywen-Revolt/wiki)

## Support
For help or questions reguarding the usage or development of Reywen please create an issue on github, its the easiest and fastest way to improve issues
