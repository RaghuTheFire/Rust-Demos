use std::net::UdpSocket;
use std::thread;
use std::sync::Arc;

fn main() 
{
    let socket = UdpSocket::bind("127.0.0.1:34254").expect("Couldn't bind to address");
    socket.set_nonblocking(true).expect("Couldn't set non-blocking");

    let socket = Arc::new(socket);
    let mut handles = vec![];

    for _ in 0..4 {
        let socket_clone = Arc::clone(&socket);
        let handle = thread::spawn(move || {
            let mut buf = [0; 1024];
            loop {
                match socket_clone.recv_from(&mut buf) {
                    Ok((size, addr)) => {
                        let data = &buf[..size];
                        println!("Received {} bytes from {}: {:?}", size, addr, data);
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // No data received, continue
                    }
                    Err(e) => {
                        eprintln!("Error receiving data: {}", e);
                    }
                }
            }
        });
        handles.push(handle);
  }

    for handle in handles 
    {
        handle.join().expect("Thread panicked");
    }
}

