use dotenvy::dotenv;
use frankenstein::{
    Api, GetUpdatesParams, ReplyParameters, SendMessageParams, TelegramApi, UpdateContent,
};
use lazy_static::lazy_static;
use local_ip_address::local_ip;
use std::env;
use std::thread::sleep;
use std::time::Duration;

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
                "--polling" => {
                    let mut update_id = 0;
                    loop {
                        update_id = polling(update_id);
                        sleep(Duration::from_secs(1));
                    }
                }
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

/// polling mode
/// return next update_id
fn polling(offset: u32) -> u32 {
    let update_params_builder = GetUpdatesParams::builder();
    let update_params = update_params_builder.clone().offset(offset).build();

    let mut last_update_id: u32 = 0;
    let result = Api::new(*TOKEN).get_updates(&update_params);
    // check response is successful or fail
    match result {
        Ok(response) => {
            // handle message(s)
            for new_message_update in response.result {
                if let UpdateContent::Message(message) = new_message_update.content {
                    let reply_message = format!(
                        "message_id: {}\nchat_id: {}",
                        message.message_id, message.chat.id
                    );
                    let reply_params = ReplyParameters::builder()
                        .message_id(message.message_id)
                        .build();
                    let send_message_params = SendMessageParams::builder()
                        .chat_id(message.chat.id)
                        .text(reply_message)
                        .reply_parameters(reply_params)
                        .build();
                    if let Err(err) = Api::new(*TOKEN).send_message(&send_message_params) {
                        println!("Failed to send message: {err:?}");
                    }
                }
                last_update_id = new_message_update.update_id;
            }
        }
        Err(error) => {
            eprintln!("Failed to get update: {error:?}");
        }
    }
    last_update_id + 1
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
