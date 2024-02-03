use std::env;
use dotenvy::dotenv;
use frankenstein::{Api, SendMessageParams, TelegramApi};
use local_ip_address::local_ip;

fn main() {
    // If .env file exists, load into environment variable.
    dotenv().ok();
    // get config
    let token_binding = env::var("TOKEN").expect("TOKEN not set");
    let token = token_binding.as_str();
    let notify_user_id: i64 = env::var("NOTIFY_USER_ID").expect("NOTIFY_USER_ID not set").parse::<i64>().expect("NOTIFY_USER_ID type is wrong.");

    // make a message
    let message = SendMessageParams::builder()
        .chat_id(notify_user_id)
        .text(get_ip())
        .build();
    // send
    Api::new(token).send_message(&message).unwrap();
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
