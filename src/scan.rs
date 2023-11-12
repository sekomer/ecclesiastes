use std::io::Write;
use std::net::{IpAddr, SocketAddr, TcpStream, ToSocketAddrs};
use std::sync::mpsc::Sender;

pub fn scan(
    tx: Sender<u16>,
    i: u16,
    start_port: u16,
    end_port: u16,
    addr: IpAddr,
    num_threads: u16,
    socket_timeout: u64,
    shared_percent: std::sync::Arc<std::sync::Mutex<f32>>,
) {
    let mut port: u16 = start_port + i;

    loop {
        let sock_address: SocketAddr = (addr, port).to_socket_addrs().unwrap().next().unwrap();

        // - knock knock
        match TcpStream::connect_timeout(
            &sock_address,
            std::time::Duration::from_millis(socket_timeout),
        ) {
            Ok(_) => {
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        // status bar update with mutex
        let mut shared_percent = shared_percent.lock().unwrap(); // acquire the lock
        let current_percent: f32 =
            ((port - start_port) as f32 / (end_port - start_port) as f32) * 100.0;

        if *shared_percent < current_percent {
            *shared_percent = current_percent;
            print!("\r {:.2}% completed", current_percent);
            std::io::stdout().flush().unwrap();
        }

        // shitty bug, <= is not covering the last port
        if (end_port - port) < num_threads {
            break;
        }

        port += num_threads;
    }
}
