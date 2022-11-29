# Reywen 
A general purpose bot running with an asynchronous 'plugin' system, built on the RevX2 library.
Written in funny crab language :crab:.

RevX2 can be used to construct your own bot!


### Main features RevX2

- async single threaded
- serde_json (ajson was used for RevX1)
- modular rust crates as 'plugins'
- masqurade (properly implemented)
- replies with mentioning
- Websocket to API struct conversion

### New features comming soon!
- Mongodb support
- Alpine based docker container
- Botctl (command line for changing json values)

### Things i won't add
- SQL support (just use mongo)
- Discord integration
- Embed support
- Moderation features (outside of RevX2)


# Getting started
The main config file needs to be linked to a bot.
Both the bot token and ID can be found [here](https://app.revolt.chat/settings/bots)

Sudoers is an optional vector of user IDs, users that are sudoers have access to more commands than regular users
```json
{

	"token": "",
	"bot_id": "",
	"sudoers": []

}
```
Additionally, each plugin has its own config under `config/*.json`.
By default every plugin besides Message is disabled.

```bash
git clone https://github.com/toastxc/Reywen-Revolt.git
cd Reywen-Revolt
nohup cargo build --release &
vim reywen.json
```


For further information of [Revolt](https://developers.revolt.chat) or [Reywen](https://github.com/toastxc/Reywen-Revolt/issues)
