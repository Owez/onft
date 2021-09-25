//! Complete project nicknamed "GameChain" which creates a single-threaded tcp
//! api, insecure and shouldn't be used in production

use onft::prelude::*;
use std::io::{self, Read};
use std::net::{TcpListener, TcpStream};
use std::{convert::TryInto, fmt, ops::Range};

const BIND_ADDR: &str = "0.0.0.0:8080";

fn main() -> io::Result<()> {
    println!("Starting TCP-based server on {} address..", BIND_ADDR);

    let listener = TcpListener::bind(BIND_ADDR)?;
    let mut chain = Chain::default();

    for stream in listener.incoming() {
        match handle_stream(&mut chain, stream?) {
            Ok(game_report) => println!("{}", game_report),
            Err(_) => continue,
        }
    }
    Ok(())
}

/// Handles an incoming tcp stream and returns a game report if successful
fn handle_stream(chain: &mut Chain, mut stream: TcpStream) -> Result<GameReport, ()> {
    // make packet then read
    let mut packet = [0; GameReport::PACKET_LEN];
    stream.read(&mut packet).map_err(|_| ())?;

    // deserialize packet then add block
    let game_report = GameReport::from_packet(packet)?;
    chain.add_block(&packet[..]).map_err(|_| ())?;

    Ok(game_report)
}

/// Report of a player's state within a game for long-term safe keeping
struct GameReport {
    game_id: u16,
    user_id: u16,
    won: bool,
}

impl GameReport {
    /// Known length of a report' packet array in bytes
    const PACKET_LEN: usize = 2 + 2 + 1;

    /// Creates new game report from inputted `packet` array
    fn from_packet(packet: [u8; Self::PACKET_LEN]) -> Result<Self, ()> {
        let get_u16 = |range: Range<usize>| u16::from_be_bytes(packet[range].try_into().unwrap());

        let game_id = get_u16(0..2);
        let user_id = get_u16(2..4);
        let won = packet[4] == 255;

        Ok(Self {
            game_id,
            user_id,
            won,
        })
    }
}

impl fmt::Display for GameReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let won = if self.won { "won" } else { "lost" };
        write!(f, "User {} {} game #{}!", self.user_id, won, self.game_id)
    }
}
