use std::net::{UdpSocket, SocketAddr};
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:34254").expect("Could not bind socket");
    socket.set_nonblocking(true).expect("Could not set non-blocking");

    let socket = Arc::new(Mutex::new(socket));
    let mut handles = vec![];

    for _ in 0..4 {
        let socket_clone = Arc::clone(&socket);
        let handle = thread::spawn(move || {
            let mut buf = [0; 1024];
            loop {
                let (size, src) = {
                    let socket = socket_clone.lock().unwrap();
                    match socket.recv_from(&mut buf) {
                        Ok((size, src)) => (size, src),
                        Err(_) => continue,
                    }
                };
                let data = &buf[..size];
                println!("Received {} bytes from {}: {:?}", size, src, data);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}

