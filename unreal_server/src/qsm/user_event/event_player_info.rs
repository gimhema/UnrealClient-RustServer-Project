
use crate::event::event_handler::EventHeader;
use crate::qsm::qsm::GLOBAL_MESSAGE_TX_QUEUE;
use crate::network::connection::MessageToSend;
use crate::qsm::user_message::message_set_user_profile_info::SetPlayerInfo;
use super::network::server_common::*;

pub fn Callback_SetPlayerInfo(buffer: &[u8]) {
 
    match SetPlayerInfo::deserialize(buffer) {
        Ok(set_player_info_msg) => {
            let player_id = set_player_info_msg.pId;
            let player_name = set_player_info_msg.UserProfileName.clone();
            
            if let Err(_) = GLOBAL_SERVER_ACTION_QUEUE.push( ServerActionType::SetPlayerProfile(player_id, player_name) ) {
                eprintln!("Failed to queue Server Action EnterPlayer");
            } else {
                println!("Queued Server Action EnterPlayer");
            }


        }
        Err(e) => {
            eprintln!("Failed to deserialize SetPlayerInfo: {}", e);
        }
    }   

}

