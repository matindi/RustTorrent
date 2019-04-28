extern crate resolve;
use resolve::{DnsConfig, DnsResolver};
use url::{Url, Host};


pub fn lookup(host : &String) -> String {
     let config = DnsConfig::with_name_servers(vec![
        // Use Google's public DNS servers instead of the system default.
        "8.8.8.8:53".parse().unwrap(),
        "8.8.4.4:53".parse().unwrap(),
    ]);

    let resolver = match DnsResolver::new(config) {
        Ok(r) => r,
        Err(e) => {
            println!("failed to create DNS resolver: {}", e);
            return String::from("null");
        }
    };

        match resolver.resolve_host(&host) {
            Ok(mut addrs) => {
                let addr = addrs.next().expect("empty ResolveHost");
                let n = addrs.count();
                return addr.to_string();
            }
            Err(e) =>  return String::from("null")
        }
        }

//      let args  = 
//         for arg in &hosts[1..] {
//             let issue_list_url = match Url::parse(&arg) {
//         Ok(base) => base,
//         Err(message) => panic!("Error parsing base {}", message)
//     };
//    let item : String = issue_list_url.host().unwrap().to_string();
//         match resolver.resolve_host(&item) {
//             Ok(mut addrs) => {
//                 let addr = addrs.next().expect("empty ResolveHost");
//                 let n = addrs.count();

//                 if n == 0 {
//                     println!("\"{}\" resolved to {}", arg, addr);
//                 } else {
//                     println!("\"{}\" resolved to {} ({} more)", arg, addr, n);
//                 }
//             }
//             Err(e) => println!("failed to resolve \"{}\": {}", arg, e)
//         }
//         };
    