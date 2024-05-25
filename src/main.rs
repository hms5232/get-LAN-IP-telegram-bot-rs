mod polling;

use dotenvy::dotenv;
use frankenstein::{Api, SendMessageParams, TelegramApi};
use lazy_static::lazy_static;
use local_ip_address::local_ip;
use std::env;

lazy_static! {
    static ref NOTIFY_USER_ID: i64 = {
        // If .env file exists, load into environment variable.
        dotenv().ok();

        env::var("NOTIFY_USER_ID")
        .expect("NOTIFY_USER_ID not set")
        .parse::<i64>()
        .expect("NOTIFY_USER_ID type is wrong.")
    };

    static ref TOKEN: &'static str = {
        // If .env file exists, load into environment variable.
        dotenv().ok();

        let token_binding: String = env::var("TOKEN").expect("TOKEN not set");
        Box::leak(token_binding.into_boxed_str())
    };
}

fn main() {
    // get command line arguments
    let args: Vec<String> = env::args().collect();
    // If no more argument, get and send IP
    // If has argument(s), check and do some action
    match args.get(1) {
        None => send_ip(),
        Some(arg) => {
            let arg_str = arg.as_str();
            match arg_str {
                // --polling => polling mod
                "--polling" => polling::run(),
                _ => println!("Unknown or unsupported args"),
            }
        }
    }
}

/// Get local IP address
fn get_ip() -> String {
    local_ip().unwrap().to_string()
}

/// Send IP to Telegram
fn send_ip() {
    // make a message
    let message = SendMessageParams::builder()
        .chat_id(*NOTIFY_USER_ID)
        .text(get_ip())
        .build();
    // send
    Api::new(*TOKEN).send_message(&message).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_ip_v4() {
        assert!(get_ip().contains("."));
        assert_eq!(4, get_ip().split(".").collect::<Vec<_>>().len());
    }
}
