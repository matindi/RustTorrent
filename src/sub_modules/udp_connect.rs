use bytes::BufMut;
use dns_lookup::lookup_host;
use laminar::{Config, Packet, Socket, SocketEvent};
use std::net::SocketAddr;
use std::thread;
use rand::Rng;



const SERVER_ADDR: &'static str = "tracker.coppersurfer.tk:6969";


pub fn send_data() {
    // Setup a udp socket and bind it to the client address.
    let (mut socket, packet_sender, event_receiver) = Socket::bind(server_address()).unwrap();
    let _thread = thread::spawn(move || socket.start_polling());

    let packet = construct_packet();

    // // next send or packet to the endpoint we earlier putted into the packet.
    packet_sender.send(packet).unwrap();


    // Next start receiving.
    let result = event_receiver.recv();

    match result {
        Ok(SocketEvent::Packet(packet)) => {
            let endpoint: SocketAddr = packet.addr();
            let received_data: &[u8] = packet.payload();
            println!("{:x?}",received_data);
            // you can here deserialize your bytes into the data you have passed it when sending.

            println!(
                "Received packet from: {:?} with length {}",
                endpoint,
                received_data.len()
            );
        }
        Ok(_) => {}
        Err(e) => {
            println!("Something went wrong when receiving, error: {:?}", e);
        }
    }
}



pub fn server_address() -> SocketAddr {
    // let ips: Vec<std::net::IpAddr> = lookup_host(SERVER_ADDR).unwrap();
    // let mut iter = ips.iter();
    // iter.next();
    return String::from("188.241.58.209:6969").parse().unwrap();
}


pub fn construct_packet() -> Packet {
    let mut buf = vec![];
    buf.put_u64_be(0x41727101980);
    buf.put_u64_be(0);
      let random_number: u32 = rand::thread_rng().gen();
    //buf.put_u64_be(random_bytes);
    //assert_eq!(buf, b"\x08\x09\xA0\xA1");
    // buf.put_u64::<BigEndian>(0);
    buf.put_u32_be(random_number);
   println!("{:x?}",buf);
    //println!("{:?}",random_number);
    // this is the destination address of the packet.
    let destination: SocketAddr = server_address();

    // lets construct some payload (raw data) for or packet.


    // lets construct or packet by passing in the destination for this packet and the bytes needed to be send..
    let packet: Packet = Packet::reliable_unordered(destination, buf.to_owned());

    packet
}

// pub fn receive_data() {
//     // setup an udp socket and bind it to the client address.
//     let (mut socket, _packet_sender, event_receiver) = Socket::bind(server_address()).unwrap();
//     let _thread = thread::spawn(move || socket.start_polling());

//     // Next start receiving.
//     let result = event_receiver.recv();

//     match result {
//         Ok(SocketEvent::Packet(packet)) => {
//             let endpoint: SocketAddr = packet.addr();
//             let received_data: &[u8] = packet.payload();

//             // you can here deserialize your bytes into the data you have passed it when sending.

//             println!(
//                 "Received packet from: {:?} with length {}",
//                 endpoint,
//                 received_data.len()
//             );
//         }
//         Ok(_) => {}
//         Err(e) => {
//             println!("Something went wrong when receiving, error: {:?}", e);
//         }
//     }
// }