pub mod models;
pub mod utils;

use serde_bencode::de::from_bytes;
use serde_bencode::value::Value as BValue;

use std::env;
use std::fs::File;
use std::io::{self, Read};

use crate::utils::parse_torrent;
use crate::utils::parse_torrent_verbose;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run <PATH_TO_TORRENT_FILE>");
        eprintln!(
            "For example: cargo run ./tests/fixtures/torrents/not-working-with-two-nodes.torrent"
        );
        std::process::exit(1);
    }

    let mut file = File::open(&args[1])?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    println!("Decoding torrent with verbose implementation ...\n");

    match from_bytes::<BValue>(&bytes) {
        Ok(value) => {
            let torrent = parse_torrent_verbose::decode_torrent(value);
            println!("Final parsed torrent: \n\n{torrent:#?}");
            Ok(())
        }
        Err(e) => {
            eprintln!("Error: {e:#?}");
            Err(io::Error::new(io::ErrorKind::Other, e))
        }
    }?;

    println!("\nDecoding torrent with standard serde implementation ...\n");

    match from_bytes::<BValue>(&bytes) {
        Ok(_value) => {
            let torrent = parse_torrent::decode_torrent(&bytes);
            println!("Final parsed torrent: \n\n{torrent:#?}");
            Ok(())
        }
        Err(e) => {
            eprintln!("Error: {e:#?}");
            Err(io::Error::new(io::ErrorKind::Other, e))
        }
    }
}
