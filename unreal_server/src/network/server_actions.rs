use std::thread::sleep;
use std::time::Duration;

use crate::qsm;
use crate::network::server::*;
use crate::network::connection::*;
use crate::qsm::user_message::message_update_players_info::UpdatePlayersInfo;
use super::qsm::user_message::message_allow_connect::*;
use crate::event::event_handler::EventHeader;
use crate::network::server_common::ServerActionType::*;
use crate::game_logic::game_player::VECharcater;
use mio::Token;
use crate::game_logic::game_player::GameNetStatus;
use crate::Command;


impl Server {

    // 대기열의 스트림으로부터 인증을 요구하는 함수
    pub fn processing_waiting_queue(&mut self) {

        let waiting_queue = self.player_waiting_queue.lock().unwrap();
        let container = waiting_queue.waiting_containter.read().unwrap();

        for token in container.iter() {
            if let Some(client) = self.clients.get_mut(token) {
                sleep(Duration::from_millis(100));
                println!("Processing client with token: {:?}", token);

                let allow_connect_message = AllowConnectGame::new(
                    EventHeader::ALLOW_CONNECT_GAME as u32,
                    0,
                    token.0 as u32,
                    "".to_string(),
                    "".to_string(),
                    "".to_string()
                );

                let send_msg = allow_connect_message.serialize();
                let req_enter_message = MessageToSend::Single(*token, send_msg);

                if let Err(_) = self.send_tcp_message(req_enter_message) {
                    eprintln!("Failed to send message to client with token: {:?}", token);
                    waiting_queue.remove(*token);
                }
            } else {
                eprintln!("Client with token {:?} not found in clients map.", token);
                waiting_queue.remove(*token);
                }
                // 
        }
}


    // 인증을 받은 플레이어를 실질적으로 통과시키는 함수
    pub fn server_action_enter_player(&mut self, 
        _pid : u32, _account_id: String, _player_name: String, _conn_info: String) {
        // Create Character by Token
        println!("Entering player with PID: {}, Account ID: {}, Player Name: {}, Conn Info: {}",
                 _pid, _account_id, _player_name, _conn_info);

        // 대기열 큐에서 제거해야함
        let waiting_queue = self.player_waiting_queue.lock().unwrap();
        
        // token is pid
        let target_token = Token(_pid as usize);
        let _pid_token = target_token.clone();
        waiting_queue.remove(target_token);
        
        // Create a new player character
        let mut new_player = VECharcater::new_zero();
        // setting network config
        new_player.player_network_config.set_sessionid(0 as i64);
        new_player.player_network_config.set_net_token(_pid_token);
        new_player.player_network_config.set_net_status(GameNetStatus::CONNECTED);


        // setting personal info
        new_player.set_player_name(_player_name.clone());
        new_player.set_player_pid(_pid as i64);
        new_player.set_player_ip_addr(_conn_info);
        self.game_character_manager.lock().unwrap().new_character(_pid as i64, new_player);

        // add new entity to game logic
        let _new_entity_id = _pid as u32;
        self.game_logic.lock().unwrap().push_command(
            Command::Create { entity_id: _new_entity_id  }
        );
    }


    pub fn server_action_set_player_profile(&mut self,_pId: u32, _player_name: String) {
            // Update Player Profile Info
            if let Some(mut character) = self.game_character_manager.lock().unwrap().get_character_mut(_pId as i64) {
                character.set_player_name(_player_name.clone());
                println!("Updated player profile for PID: {} with new name: {}", _pId, _player_name);


                let update_players_info_message = UpdatePlayersInfo::new(
                    EventHeader::UPDATE_PLAYERS_INFO as u32, 
                    _pId, 
                    _player_name);

                let send_msg = update_players_info_message.serialize();
//                let req_enter_message = MessageToSend::Single(*token, send_msg);
                let req_enter_message = MessageToSend::Broadcast(send_msg);

                if let Err(_) = self.send_tcp_message(req_enter_message) {
//                    eprintln!("Failed to send message to client with token: {:?}", token);
                }
                else {
                }


            } else {
                eprintln!("Character with PID: {} not found for profile update.", _pId);
            }
    }

}
