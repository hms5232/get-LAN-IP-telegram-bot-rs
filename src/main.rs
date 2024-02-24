use dotenvy::dotenv;
use frankenstein::{Api, SendMessageParams, TelegramApi};
use local_ip_address::local_ip;
use std::env;
use lazy_static::lazy_static;

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
    // make a message
    let message = SendMessageParams::builder()
        .chat_id(*NOTIFY_USER_ID)
        .text(get_ip())
        .build();
    // send
    Api::new(*TOKEN).send_message(&message).unwrap();
}

/// Get local IP address
fn get_ip() -> String {
    local_ip().unwrap().to_string()
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
