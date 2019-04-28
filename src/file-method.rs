use bendy::{
    decoding::{Error, FromBencode, Object, ResultExt},
    encoding::AsString,
};
use std::net::UdpSocket;
use std::time::{Duration, Instant};
use std::io::ErrorKind;
use std::thread;


static EXAMPLE_TORRENT: &[u8] =
    include_bytes!("C:/Users/brian/Documents/projects/rustApps/torrent-app/src/debian-9.4.0-amd64-netinst.iso.torrent");
#[derive(Debug)]
struct MetaInfo {
    pub announce: String,
    pub info: Info,
    pub comment: Option<String>,         // not official element
    pub created_by: Option<String>,         // not official element
    pub creation_date: Option<u64>,      // not official element
    pub http_seeds: Option<Vec<String>>, // not official element
    pub encoding: Option<String>,
}

/// File related information (Single-file format)
#[derive(Debug)]
struct Info {
    pub piece_length: String,
    pub pieces: Vec<u8>,
    pub name: String,
    pub file_length: String,
}

impl FromBencode for MetaInfo {
    const EXPECTED_RECURSION_DEPTH: usize = Info::EXPECTED_RECURSION_DEPTH + 1;
    fn decode_bencode_object(object: Object) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut announce = None;
        let mut comment = None;
        let mut creation_date = None;
        let mut http_seeds = None;
        let mut info = None;
        let mut created_by = None;
        let mut encoding = None;

        let mut dict_dec = object.try_into_dictionary()?;
        while let Some(pair) = dict_dec.next_pair()? {
            match pair {
                (b"announce", value) => {
                    announce = String::decode_bencode_object(value)
                        .context("announce")
                        .map(Some)?;
                },
                (b"comment", value) => {
                    comment = String::decode_bencode_object(value)
                        .context("comment")
                        .map(Some)?;
                },
                (b"encoding", value) => {
                    encoding = String::decode_bencode_object(value)
                        .context("encoding")
                        .map(Some)?;
                },
                (b"created by", value) => {
                    created_by = String::decode_bencode_object(value)
                        .context("created by")
                        .map(Some)?;
                },
                (b"creation date", value) => {
                    creation_date = u64::decode_bencode_object(value)
                        .context("creation_date")
                        .map(Some)?;
                },
                (b"httpseeds", value) => {
                    http_seeds = Vec::decode_bencode_object(value)
                        .context("http_seeds")
                        .map(Some)?;
                },
                (b"info", value) => {
                    info = Info::decode_bencode_object(value)
                        .context("info")
                        .map(Some)?;
                },
                (unknown_field, _) => {
                    return Err(Error::unexpected_field(String::from_utf8_lossy(
                        unknown_field,
                    )));
                },
            }
        }

        let announce = announce.ok_or_else(|| Error::missing_field("announce"))?;
        let info = info.ok_or_else(|| Error::missing_field("info"))?;

        Ok(MetaInfo {
            announce,
            info,
            comment,
            creation_date,
            http_seeds,
            created_by,
            encoding,
        })
    }
}

impl FromBencode for Info {
    const EXPECTED_RECURSION_DEPTH: usize = 1;
    fn decode_bencode_object(object: Object) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut file_length = None;
        let mut name = None;
        let mut piece_length = None;
        let mut pieces = None;

        let mut dict_dec = object.try_into_dictionary()?;
        while let Some(pair) = dict_dec.next_pair()? {
            match pair {
                (b"length", value) => {
                    file_length = value
                        .try_into_integer()
                        .context("file.length")
                        .map(ToString::to_string)
                        .map(Some)?;
                },
                (b"name", value) => {
                    name = String::decode_bencode_object(value)
                        .context("name")
                        .map(Some)?;
                },
                (b"piece length", value) => {
                    piece_length = value
                        .try_into_integer()
                        .context("length")
                        .map(ToString::to_string)
                        .map(Some)?;
                },
                (b"pieces", value) => {
                    pieces = AsString::decode_bencode_object(value)
                        .context("pieces")
                        .map(|bytes| Some(bytes.0))?;
                },
                (unknown_field, _) => {
                    return Err(Error::unexpected_field(String::from_utf8_lossy(
                        unknown_field,
                    )));
                },
            }
        }

        let file_length = file_length.ok_or_else(|| Error::missing_field("file_length"))?;
        let name = name.ok_or_else(|| Error::missing_field("name"))?;
        let piece_length = piece_length.ok_or_else(|| Error::missing_field("piece_length"))?;
        let pieces = pieces.ok_or_else(|| Error::missing_field("pieces"))?;
        Ok(Info {
            file_length,
            name,
            piece_length,
            pieces,
        })
    }
}

fn main() -> Result<(), Error> {
    let torrent = MetaInfo::from_bencode(EXAMPLE_TORRENT)?;
    println!("{:#?}", torrent.announce);
    //   let sock = UdpSocket::bind(torrent.announce).expect("Failed to bind socket");
    // sock.set_nonblocking(true)
    //     .expect("Failed to enter non-blocking mode");
    Ok(())
}