use std::thread::sleep;
use std::time::Duration;

use frankenstein::{
    Api, GetUpdatesParams, ReplyParameters, SendMessageParams, TelegramApi, UpdateContent,
};

use crate::TOKEN;

pub fn run() {
    ctrlc::set_handler(move || {
        println!("Stop bot polling mode");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let mut update_id = 0;
    loop {
        update_id = get_update_then_reply(update_id);
        sleep(Duration::from_secs(1));
    }
}

/// Get new message and reply information
/// return next update_id
pub(crate) fn get_update_then_reply(offset: u32) -> u32 {
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
                    dbg!(&message);

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
