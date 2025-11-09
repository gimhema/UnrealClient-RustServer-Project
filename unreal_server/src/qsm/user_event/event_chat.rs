
use crate::qsm::user_message::message_chat::{self, ChatMessage};
use super::Network::server_common::*;
use crate::Event::event_handler::EventHeader;
use crate::qsm::qsm::GLOBAL_MESSAGE_TX_QUEUE;
use crate::Network::connection::MessageToSend;
pub fn CallBack_Chat(buffer: &[u8])
{
    match ChatMessage::deserialize(buffer) {
        Ok(chat_message) => {
            let sender = chat_message.id;
            let chat_content = chat_message.content;

            // let client_token = Token(client_token_value.try_into().unwrap());

            println!("sender id : {}", sender);
            println!("chat content : {}", chat_content);

            let mut _chat_message = ChatMessage::new(
                EventHeader::CHAT_MESSAGE as u32,
                sender,
                chat_content.clone(),
            );

            let send_msg = _chat_message.serialize();

            if let Err(_) = GLOBAL_MESSAGE_TX_QUEUE.push(MessageToSend::Broadcast(send_msg)) {
                // eprintln!("Failed to queue AllowConnectGame message for client {:?}", client_token);
            } else {
                // println!("Queued AllowConnectGame message for client {:?}", client_token);
            }
        }
        Err(e) => {
            eprintln!("Failed to deserialize ChatMessage: {}", e);
        }
    }
}



