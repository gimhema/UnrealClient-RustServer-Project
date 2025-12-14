// use crate::get_udp_server_instance;
use crate::qsm::{qsm::GLOBAL_MESSAGE_UDP_QUEUE, user_message::message_movement::{self, PlayerMovement}};

use super::game_logic::game_logic_main::*;
use super::game_logic::game_logic_handle::get_game_logic;

pub fn CallBack_PlayerMovementUpdate(buffer: &[u8])
{
    match PlayerMovement::deserialize(buffer ) {
        Ok(movement_message) => {
            let sender = movement_message.id;
            let loc_x = movement_message.x;
            let loc_y = movement_message.y;
            let loc_z = movement_message.z;
            let roll = movement_message.roll;
            let pitch = movement_message.pitch;
            let yaw = movement_message.yaw;

            if let Some(gl_arc) = get_game_logic() {
                if let Ok(mut gl) = gl_arc.lock() {
                    gl.push_command(
                        Command::Move {
                            entity_id: sender,
                            loc_x, loc_y, loc_z,
                            q_x: roll, q_y: pitch, q_z: yaw, q_w: 0.0,
                        }
                    );
                } else {
                    eprintln!("[MovementCB] Failed to lock game_logic.");
                }
            } else {
                eprintln!("[MovementCB] game_logic not initialized (set_global_game_logic missing).");
            }

        }
        Err(e) => {
            eprintln!("Failed to deserialize MovementMessage: {}", e);
        }
    }
}

pub fn CallBack_AddPlayerInput(buffer: &[u8])
{
    
}


