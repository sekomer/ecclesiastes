use argparse::{ArgumentParser, Store, StoreOption};
use std::net::IpAddr;
use std::str::FromStr;

pub fn parse_args() -> (String, u16, u16, u64, u16) {
    let mut port_interval = String::new();
    let mut ip_address = String::new();
    let mut socket_timeout: Option<u64> = None;
    let mut num_threads: Option<u16> = None;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("simple ip sniffer writtin in rust");

        ap.refer(&mut num_threads)
            .add_option(&["-j", "--jobs"], StoreOption, "Number of threads");
        ap.refer(&mut socket_timeout).add_option(
            &["-t", "--timeout"],
            StoreOption,
            "Timeout for socket connection in ms, default is 1000",
        );
        ap.refer(&mut ip_address)
            .add_argument("ip_address", Store, "IP address");
        ap.refer(&mut port_interval).add_argument(
            "port_interval",
            Store,
            "Port interval, e.g. `1:9`, default is `1:65535`",
        );

        ap.parse_args_or_exit();
    }

    // check if ip_address is valid
    match IpAddr::from_str(&ip_address) {
        Ok(_) => {}
        Err(_) => {
            println!("Not a valid IP address");
            std::process::exit(0);
        }
    }

    let mut num_threads = match num_threads {
        Some(n) => n,
        None => 1024,
    };

    let socket_timeout = match socket_timeout {
        Some(n) => n,
        None => 1000,
    };

    // if port_interval is empty, set to default
    if port_interval.is_empty() {
        port_interval = String::from("0:65535");
    }

    // check if port_interval is valid, convert it to 2 u16 values
    let port_interval: Vec<&str> = port_interval.split(":").collect();
    if port_interval.len() != 2 {
        println!("Not a valid port interval");
        std::process::exit(0);
    }
    let port_interval: Vec<u16> = port_interval
        .iter()
        .map(|x| {
            x.parse::<u16>().unwrap_or_else(
                // if port_interval is not a number throw error
                |_| {
                    println!("Not a valid port interval");
                    std::process::exit(0);
                },
            )
        })
        .collect();

    if port_interval[0] > port_interval[1] {
        println!("Not a valid port interval");
        std::process::exit(0);
    }

    if port_interval[1] - port_interval[0] < num_threads {
        // reduce num_threads if port_interval is smaller
        num_threads = port_interval[1] - port_interval[0] + 1;
    }

    return (
        ip_address,
        port_interval[0],
        port_interval[1],
        socket_timeout,
        num_threads,
    );
}
