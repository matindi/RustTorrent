use url::percent_encoding::percent_decode;
use url::{Url, ParseError};
use super::dns_lookup;

pub fn parse_torrent(s: &String) -> Vec<String> {
    let mut miniuris: Vec<MiniUri> = Vec::new();
    let token: char = '&';
    let split = s.split(token);
    let vec = split.collect::<Vec<&str>>();
    let mut v1_iter = vec.iter();

    loop {
        match v1_iter.next() {
            Some(number) => {
                miniuris.push(split_uri(number));
            }
            None => break,
        }

    }
    let last = MagnetUri::new(&miniuris);
    println!("{:?}", last);

    // if split_uri(number).name == "tr"{
    //      println!("{:?}", split_uri(number));
    //          miniuris.push(split_uri(number));
    // }
    return last.address_tracker;
}

pub fn split_uri(s: &str) -> MiniUri {
    let token: char = '=';
    let split = s.split(token);
    let vec = split.collect::<Vec<&str>>();
    MiniUri::new(vec[0], vec[1])
}


#[derive(Debug)]
pub struct MagnetUri {
    display_name: String,
    exact_length: String,
    exact_topic: String,
    acceptable_source: String,
    exact_source: String,
    keyword_topic: String,
    manifest_topic: String,
    address_tracker: Vec<String>,
    torrent_url : Vec<TorrentUrl>,
}
impl MagnetUri {
    pub fn new(miniuris: &Vec<MiniUri>) -> MagnetUri {
        let mut display_name: String = String::from("");
        let mut exact_length: String = String::from("");
        let mut exact_topic: String = String::from("");
        let mut acceptable_source: String = String::from("");
        let mut exact_source: String = String::from("");
        let mut keyword_topic: String = String::from("");
        let mut manifest_topic: String = String::from("");
        let mut address_tracker: Vec<String> = Vec::new();
        let mut torrent_url: Vec<TorrentUrl> = Vec::new();
        for i in 0..miniuris.len() {
            if miniuris[i].name == "dn" {
                display_name = miniuris[i].value.clone();
            } else if miniuris[i].name == "tr" {
                let address =percent_decode((miniuris[i].value.clone()).as_bytes())
                        .decode_utf8()
                        .unwrap()
                        .to_string();
                        
                let issue_list_url = match Url::parse(&address) {
        Ok(base) => base,
        Err(message) => panic!("Error parsing base {}", message)
    };
       let ip : String = dns_lookup::lookup(&issue_list_url.host().unwrap().to_string());
       let port : String = issue_list_url.port().unwrap().to_string();
       let torrent_url1 = TorrentUrl::new(&ip, &port);
                torrent_url.push(torrent_url1);
                address_tracker.push(address
                );
            }
        }
        return MagnetUri {
            display_name,
            exact_length,
            exact_topic,
            acceptable_source,
            exact_source,
            keyword_topic,
            manifest_topic,
            address_tracker,
            torrent_url,
        };
    }

}
#[derive(Debug)]
pub struct MiniUri {
    name: String,
    value: String,
}

#[derive(Debug)]
pub struct TorrentUrl{
    ip_address : String,
    port : String,
}
impl TorrentUrl {
    pub fn new(ip_address: &str, port: &str) -> TorrentUrl {
        TorrentUrl {
            ip_address: ip_address.to_string(),
            port: port.to_string(),
        }
    }
}

impl MiniUri {
    pub fn new(name: &str, value: &str) -> MiniUri {
        MiniUri {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

