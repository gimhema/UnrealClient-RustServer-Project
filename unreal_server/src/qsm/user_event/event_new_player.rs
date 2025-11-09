use crate::qsm::user_message::message_allow_connect::*;
use crate::qsm::user_message::message_server_response::{self, ServerResponse};

use super::Network::server_common::*;

pub fn CallBack_CreateNewPlayer(buffer: &[u8])
{

}

pub fn CallBack_EnterNewPlayerToGame(buffer: &[u8])
{

}


pub fn Callback_SetPlayerInfo(buffer: &[u8])
{

}

pub fn CallBack_AllowConnectGame(buffer: &[u8])
{
    match AllowConnectGame::deserialize(buffer) {
        Ok(allow_connect_message) => {
            let _pid = allow_connect_message.pid;
            let _account_id = allow_connect_message.accountId.clone();
            let _player_name = allow_connect_message.name.clone();
            let _conn_info = allow_connect_message.connect_info.clone();
            
            
            println!("CallBack_AllowConnectGame : PID : {}, Account ID : {}, Player Name : {}, Conn: {}",
                     _pid, _account_id, _player_name, _conn_info);


            if let Err(_) = GLOBAL_SERVER_ACTION_QUEUE.push(ServerActionType::EnterPlayer(_pid, _account_id, _player_name, _conn_info)) {
                eprintln!("Failed to queue Server Action EnterPlayer");
            } else {
                println!("Queued Server Action EnterPlayer");
            }
        }
        Err(e) => {
            eprintln!("Failed to deserialize AllowConnectGame: {}", e);
        }
    }
}