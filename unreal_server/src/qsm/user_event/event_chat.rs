
use crate::qsm::user_message::message_chat::{self, ChatMessage};
use super::Network::server_common::*;



pub fn CallBack_Chat(buffer: &[u8])
{
    match ChatMessage::deserialize(buffer) {
        Ok(chat_message) => {
            let sender = chat_message.id;
            let chat_content = chat_message.content;

            println!("sender id : {}", sender);
            println!("chat content : {}", chat_content);

            if let Err(_) = GLOBAL_SERVER_ACTION_QUEUE.push(ServerActionType::ChatMessage(sender, chat_content)) {
                eprintln!("Failed to queue Server Action EnterPlayer");
            } else {
                println!("Queued Server Action EnterPlayer");
            }
        }
        Err(e) => {
            eprintln!("Failed to deserialize ChatMessage: {}", e);
        }
    }
}



