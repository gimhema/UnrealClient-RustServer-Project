
use crate::Event::event_handler::EventHeader;
use crate::qsm::qsm::GLOBAL_MESSAGE_TX_QUEUE;
use crate::Network::connection::MessageToSend;
use crate::qsm::user_message::message_set_user_profile_info::SetPlayerInfo;

pub fn Callback_SetPlayerInfo(buffer: &[u8]) {
 
    match SetPlayerInfo::deserialize(buffer) {
        Ok(set_player_info_msg) => {
            let player_id = set_player_info_msg.pId;
            let player_name = set_player_info_msg.UserProfileName.clone();
            
            


        }
        Err(e) => {
            eprintln!("Failed to deserialize SetPlayerInfo: {}", e);
        }
    }   

}

