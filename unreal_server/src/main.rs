// use qsm::qsm::get_event_handler;
// use network::server_datagram::get_udp_server_instance;


#[macro_use]
extern crate lazy_static;

mod agent;
mod event;
mod network;
mod qsm;
mod crypto;
mod session;
mod game_logic;

// core Logic
mod core;
use crate::game_logic::game_logic_main::*;
use crate::game_logic::game_setting::*;
use std::sync::Arc;
use std::thread;
use std::sync::Mutex;
use std::time::{Duration, Instant};
// use tokio::time::Duration;
use crate::network::message_queue::*;
use crate::network::server::*;
use tokio::io;
use mio::Token;

// // --- 메인 함수 ---
fn main() -> io::Result<()> {


    // 서버 인스턴스 생성
    let mut server = Server::new("127.0.0.1:8080", "127.0.0.1:8082")?;
    // 서버 시작
    server.start()?;

    Ok(())
}