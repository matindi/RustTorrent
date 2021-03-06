use std::net::UdpSocket;
use std::time::{Duration, Instant};
use std::io::ErrorKind;
use std::thread;

fn main() {
    let sock = UdpSocket::bind("0.0.0.0:5546").expect("Failed to bind socket");
    sock.set_nonblocking(true)
        .expect("Failed to enter non-blocking mode");

    // Poll for data every 5 milliseconds for 5 seconds.
    let start = Instant::now();
    let mut buf = [0u8; 1024];

    while start.elapsed().as_secs() < 5 {
        let result = sock.recv(&mut buf);
        match result {
            // If `recv` was successfull, print the number of bytes received.
            // The received data is stored in `buf`.
            Ok(num_bytes) => println!("I received {} bytes!", num_bytes),
            // If we get an error other than "would block", print the error.
            Err(ref err) if err.kind() != ErrorKind::WouldBlock => {
                println!("Something went wrong: {}", err)
            }
            // Do nothing otherwise.
            _ => {}
        }

        thread::sleep(Duration::from_millis(5));
    }

    println!("Done");
}
