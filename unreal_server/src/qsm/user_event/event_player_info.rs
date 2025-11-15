
use crate::Event::event_handler::EventHeader;
use crate::qsm::qsm::GLOBAL_MESSAGE_TX_QUEUE;
use crate::Network::connection::MessageToSend;

pub fn Callback_SetPlayerInfo(buffer: &[u8]) {
 
 
     println!("Received SetPlayerInfo event with buffer size: {}", buffer.len());

 }