mod sub_modules{
    pub mod udp_connect;
    pub mod parser;
    pub mod dns_lookup;
}
use std::env;


fn main() {
      let args: Vec<String> = env::args().collect();
    let query = &args[1];
 
    
   // sub_modules::udp_connect::construct_packet();
   // sub_modules::udp_connect::send_data();
   // sub_modules::udp_connect::receive_data();
    sub_modules::parser::parse_torrent(&query);
}

