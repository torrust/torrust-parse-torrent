use serde_bencode::ser;
use serde_bytes::ByteBuf;
use serde_derive::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

use crate::utils::hex::from_bytes;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Torrent {
    pub info: TorrentInfo, //
    #[serde(default)]
    pub announce: Option<String>,
    #[serde(default)]
    pub nodes: Option<Vec<TorrentNode>>,
    #[serde(default)]
    pub encoding: Option<String>,
    #[serde(default)]
    pub httpseeds: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "announce-list")]
    pub announce_list: Option<Vec<Vec<String>>>,
    #[serde(default)]
    #[serde(rename = "creation date")]
    pub creation_date: Option<i64>,
    #[serde(rename = "comment")]
    pub comment: Option<String>,
    #[serde(default)]
    #[serde(rename = "created by")]
    pub created_by: Option<String>,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct TorrentInfo {
    pub name: String,
    #[serde(default)]
    pub pieces: Option<ByteBuf>,
    #[serde(rename = "piece length")]
    pub piece_length: i64,
    #[serde(default)]
    pub md5sum: Option<String>,
    #[serde(default)]
    pub length: Option<i64>,
    #[serde(default)]
    pub files: Option<Vec<TorrentFile>>,
    #[serde(default)]
    pub private: Option<u8>,
    #[serde(default)]
    pub path: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "root hash")]
    pub root_hash: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
}

impl Default for TorrentInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            pieces: Some(ByteBuf::from(vec![])),
            piece_length: 0,
            md5sum: None,
            length: Some(0),
            files: None,
            private: None,
            path: None,
            root_hash: None,
            source: None,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct TorrentNode(pub String, pub i64);

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct TorrentFile {
    pub path: Vec<String>,
    pub length: i64,
    #[serde(default)]
    pub md5sum: Option<String>,
}

impl Default for Torrent {
    fn default() -> Self {
        Self {
            info: TorrentInfo::default(),
            announce: None,
            announce_list: Some(vec![]),
            creation_date: None,
            comment: None,
            created_by: None,
            nodes: None,
            encoding: None,
            httpseeds: None,
        }
    }
}

impl Torrent {
    /// It calculates the info hash of the torrent file.
    ///
    /// # Panics
    ///
    /// This function will panic if the `info` part of the torrent file cannot be serialized.
    #[must_use]
    pub fn calculate_info_hash_as_bytes(&self) -> [u8; 20] {
        let info_bencoded =
            ser::to_bytes(&self.info).expect("variable `info` was not able to be serialized.");
        let mut hasher = Sha1::new();
        hasher.update(info_bencoded);
        let sum_hex = hasher.finalize();
        let mut sum_bytes: [u8; 20] = Default::default();
        sum_bytes.copy_from_slice(sum_hex.as_slice());
        sum_bytes
    }

    #[must_use]
    pub fn info_hash(&self) -> String {
        // todo: return an InfoHash struct
        from_bytes(&self.calculate_info_hash_as_bytes()).to_lowercase()
    }

    #[must_use]
    pub fn file_size(&self) -> i64 {
        match self.info.length {
            Some(length) => length,
            None => match &self.info.files {
                None => 0,
                Some(files) => {
                    let mut file_size = 0;
                    for file in files {
                        file_size += file.length;
                    }
                    file_size
                }
            },
        }
    }

    /// It returns the announce urls of the torrent file.
    ///
    /// # Panics
    ///
    /// This function will panic if both the `announce_list` and the `announce` are `None`.
    #[must_use]
    pub fn announce_urls(&self) -> Vec<String> {
        match &self.announce_list {
            Some(list) => list.clone().into_iter().flatten().collect::<Vec<String>>(),
            None => vec![self
                .announce
                .clone()
                .expect("variable `announce` should not be None")],
        }
    }
}
