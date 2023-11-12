use std::net::IpAddr;
use std::str::FromStr;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;

mod parse;
mod scan;

fn main() {
    let (ip_address, start_port, end_port, socket_timeout, num_threads) = parse::parse_args();

    let mut out: Vec<u16> = vec![];

    println!(
        "[INFO] Scanning {} ports with {} worker threads",
        end_port - start_port,
        num_threads
    );

    let (tx, rx) = channel();

    let shared_percent = Arc::new(Mutex::new(0.));

    for i in 0..num_threads {
        let tx = tx.clone();
        let ip_address = ip_address.clone();
        let start_port = start_port.clone();
        let end_port = end_port.clone();
        let socket_timeout = socket_timeout.clone();
        let shared_percent = Arc::clone(&shared_percent); // Clone the Arc to share ownership

        thread::spawn(move || {
            scan::scan(
                tx,
                i,
                start_port,
                end_port,
                IpAddr::from_str(&ip_address).unwrap(),
                num_threads,
                socket_timeout,
                shared_percent,
            );
        });
    }

    drop(tx);

    for p in rx {
        out.push(p);
    }

    // this is to make sure that the progress bar is 100% completed
    println!("\r 100.00% completed\n");
    println!("[ Open Ports ]");

    out.sort();

    // prettification efforts
    for (i, v) in out.iter().enumerate() {
        print!("{:>5} ", v);

        if (i + 1) % 10 == 0 {
            println!("");
        }
    }

    println!();
    println!("[INFO] Scan finished");
}
