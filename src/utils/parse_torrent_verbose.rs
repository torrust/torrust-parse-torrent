//! Parse a torrent file data using low-level serde capabilities to show better
//! error messages.

use crate::models::torrent_file::{Torrent, TorrentFile, TorrentInfo, TorrentNode};

use serde_bencode::value::Value as BValue;
use serde_bytes::ByteBuf;

/// Parses a torrent file into a `Torrent` struct using low-level serde
/// capabilities.
///
/// # Panics
///
/// This function will panic if the torrent file is not a valid bencoded file.
#[allow(clippy::too_many_lines)]
#[allow(clippy::cast_possible_truncation)]
#[must_use]
pub fn decode_torrent(bvalue: BValue) -> Torrent {
    let mut torrent = Torrent::default();

    match bvalue {
        BValue::Dict(dict) => {
            for (key, value) in dict {
                let key = String::from_utf8_lossy(&key).into_owned();
                match &key[..] {
                    "info" => {
                        if let BValue::Dict(info_dict) = value {
                            let mut info = TorrentInfo {
                                name: String::new(),
                                pieces: None,
                                piece_length: 0,
                                md5sum: None,
                                length: None,
                                files: None,
                                private: None,
                                path: None,
                                root_hash: None,
                                source: None,
                            };
                            for (info_key, info_value) in info_dict {
                                let info_key = String::from_utf8_lossy(&info_key).into_owned();
                                match info_key.as_str() {
                                    "name" => {
                                        if let BValue::Bytes(bytes) = &info_value {
                                            info.name = String::from_utf8_lossy(bytes).into_owned();
                                        }
                                    }
                                    "pieces" => {
                                        if let BValue::Bytes(bytes) = &info_value {
                                            info.pieces = Some(ByteBuf::from(bytes.clone()));
                                            println!("Pieces length: {}", bytes.len());
                                        }
                                    }
                                    "piece length" => {
                                        if let BValue::Int(int) = info_value {
                                            info.piece_length = int;
                                        }
                                    }
                                    "md5sum" => {
                                        if let BValue::Bytes(bytes) = &info_value {
                                            info.md5sum =
                                                Some(String::from_utf8_lossy(bytes).into_owned());
                                        }
                                    }
                                    "length" => {
                                        if let BValue::Int(int) = info_value {
                                            info.length = Some(int);
                                        }
                                    }
                                    "files" => {
                                        if let BValue::List(files) = &info_value {
                                            let mut torrent_files = vec![];
                                            for file in files {
                                                if let BValue::Dict(file_dict) = file {
                                                    let mut torrent_file = TorrentFile {
                                                        path: vec![],
                                                        length: 0,
                                                        md5sum: None,
                                                    };
                                                    for (file_key, file_value) in file_dict {
                                                        let file_key =
                                                            String::from_utf8_lossy(file_key)
                                                                .into_owned();
                                                        match file_key.as_str() {
                                                            "path" => {
                                                                if let BValue::List(path_list) =
                                                                    file_value
                                                                {
                                                                    for path_item in path_list {
                                                                        if let BValue::Bytes(
                                                                            path_bytes,
                                                                        ) = path_item
                                                                        {
                                                                            torrent_file.path.push(String::from_utf8_lossy(path_bytes).into_owned());
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            "length" => {
                                                                if let BValue::Int(length) =
                                                                    file_value
                                                                {
                                                                    torrent_file.length = *length;
                                                                }
                                                            }
                                                            "md5sum" => {
                                                                if let BValue::Bytes(md5sum_bytes) =
                                                                    file_value
                                                                {
                                                                    torrent_file.md5sum = Some(
                                                                        String::from_utf8_lossy(
                                                                            md5sum_bytes,
                                                                        )
                                                                        .into_owned(),
                                                                    );
                                                                }
                                                            }
                                                            _ => {
                                                                println!(
                                                                    "Skipped file key: {file_key}"
                                                                );
                                                            }
                                                        }
                                                    }
                                                    torrent_files.push(torrent_file);
                                                }
                                            }
                                            info.files = Some(torrent_files);
                                        }
                                    }
                                    "private" => {
                                        if let BValue::Int(private) = info_value {
                                            match private {
                                                0 => info.private = Some(0),
                                                1 => info.private = Some(1),
                                                _ => {
                                                    panic!("Unexpected private value: {private}");
                                                }
                                            }
                                        }
                                    }
                                    "path" => {
                                        if let BValue::List(path_list) = &info_value {
                                            let mut path = vec![];
                                            for path_item in path_list {
                                                if let BValue::Bytes(path_bytes) = path_item {
                                                    path.push(
                                                        String::from_utf8_lossy(path_bytes)
                                                            .into_owned(),
                                                    );
                                                }
                                            }
                                            info.path = Some(path);
                                        }
                                    }
                                    "root hash" => {
                                        if let BValue::Bytes(bytes) = &info_value {
                                            info.root_hash =
                                                Some(String::from_utf8_lossy(bytes).into_owned());
                                        }
                                    }
                                    "source" => {
                                        if let BValue::Bytes(bytes) = &info_value {
                                            info.source =
                                                Some(String::from_utf8_lossy(bytes).into_owned());
                                        }
                                    }
                                    _ => {
                                        println!("Skipped info key: {info_key}");
                                    }
                                }
                            }
                            torrent.info = info;
                        }
                    }
                    "announce" => {
                        if let BValue::Bytes(bytes) = value {
                            torrent.announce = Some(String::from_utf8_lossy(&bytes).into_owned());
                        }
                    }
                    "nodes" => {
                        if let BValue::List(nodes) = &value {
                            let mut nodes_vec = vec![];
                            for node in nodes {
                                if let BValue::List(node_list) = node {
                                    if let Some(BValue::Bytes(host)) = node_list.get(0) {
                                        if let Some(BValue::Int(port)) = node_list.get(1) {
                                            nodes_vec.push(TorrentNode(
                                                String::from_utf8_lossy(host).into_owned(),
                                                *port,
                                            ));
                                        }
                                    }
                                }
                            }
                            torrent.nodes = Some(nodes_vec);
                        }
                    }
                    "encoding" => {
                        if let BValue::Bytes(bytes) = value {
                            torrent.encoding = Some(String::from_utf8_lossy(&bytes).into_owned());
                        }
                    }
                    "httpseeds" => {
                        if let BValue::List(seeds) = &value {
                            let mut httpseeds_vec = vec![];
                            for seed in seeds {
                                if let BValue::Bytes(bytes) = seed {
                                    httpseeds_vec.push(String::from_utf8_lossy(bytes).into_owned());
                                }
                            }
                            torrent.httpseeds = Some(httpseeds_vec);
                        }
                    }
                    "announce-list" => {
                        if let BValue::List(lists) = &value {
                            let mut announce_list_vec = vec![];
                            for list in lists {
                                if let BValue::List(announce_list) = list {
                                    let mut inner_vec = vec![];
                                    for announce in announce_list {
                                        if let BValue::Bytes(bytes) = announce {
                                            inner_vec
                                                .push(String::from_utf8_lossy(bytes).into_owned());
                                        }
                                    }
                                    announce_list_vec.push(inner_vec);
                                }
                            }
                            torrent.announce_list = Some(announce_list_vec);
                        }
                    }
                    "creation date" => {
                        if let BValue::Int(int) = value {
                            torrent.creation_date = Some(int);
                        }
                    }
                    "comment" => {
                        if let BValue::Bytes(bytes) = value {
                            torrent.comment = Some(String::from_utf8_lossy(&bytes).into_owned());
                        }
                    }
                    "created by" => {
                        if let BValue::Bytes(bytes) = value {
                            torrent.created_by = Some(String::from_utf8_lossy(&bytes).into_owned());
                        }
                    }
                    _ => {
                        println!("Skipped Dict key: {key}");
                    }
                };
            }
        }
        BValue::Bytes(_) => panic!("Unexpected Bytes value"),
        BValue::Int(_) => panic!("Unexpected Int value"),
        BValue::List(_) => panic!("Unexpected List value"),
    };

    torrent
}
