# get-LAN-IP-telegram-bot-rs
The Rust version of [get-LAN-IP-telegram-bot](https://github.com/hms5232/get-LAN-IP-telegram-bot).

## Background

The original project is developed for Raspberry Pi. This saves me much time from IP checking.

But, I'm not sure why the Python virtual environment can't get IP when RPi just booted. I resolved it with a stupid way: delay.

This actually work. However, I need to keep waiting  even device had booted until message delivered.

Now, I migrate this project to Rust, and it seems more fast and reliable in my case.

## Usage

### Config

You can use environment variables or dotenv.

#### Environment

```shell
export TOKEN='bot token'
export NOTIFY_USER_ID='chat id'
```

or

```shell
TOKEN='bot token' NOTIFY_USER_ID='chat id' /path/to/binary
```

#### dotenv

```shell
cp .env.example .env
```

then replace value for your bot and chat/user id.

※ The `.env` file should be placed same or parent dir with executable binary file. 

### Run on RPi

First, build binary or download pre-built binary from [release page](https://github.com/hms5232/get-LAN-IP-telegram-bot-rs/releases).

Optional, you can rename it to `get-LAN-IP-telegram-bot-rs` or other filename you like.

Add the following line into `/etc/rc.local`:

```shell
(cd /path/to/binary && ./get-LAN-IP-telegram-bot-rs; cd -)&
```

or

```shell
(cd /path/to/binary && TOKEN='bot token' NOTIFY_USER_ID='chat id' ./get-LAN-IP-telegram-bot-rs; cd -)&
```

Don't forget that replace `get-LAN-IP-telegram-bot-rs` with you just named to binary

## LICENSE

[MPL 2.0](LICENSE)
