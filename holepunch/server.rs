use std::net::{UdpSocket, SocketAddr};
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:34254")?;

    let mut clients: HashSet<SocketAddr> = HashSet::new();

    loop {
        let mut buf = [0; 128];
        let (amt, src) = socket.recv_from(&mut buf)?;
        println!("Received buffer of size {} from {}", amt, src);

        // Broadcast existing clients to new client
        for client in &clients {
            let msg = client.to_owned().to_string();
            let buf = &mut msg.as_bytes();
            socket.send_to(buf, src)?;
        }

        // Broadcast new client to all clients
        let msg = src.to_owned().to_string();
        let buf = &mut msg.as_bytes();
        for client in &clients {
            socket.send_to(buf, client)?;
        }
        clients.insert(src);

    }
}
