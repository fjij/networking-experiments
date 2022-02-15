use std::net::{UdpSocket, SocketAddr};
use std::error::Error;
use std::str;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

trait AddrReceiver {
    fn recv_addr(&self) -> Result<SocketAddr>;
}

trait StringSocket {
    fn recv_string(&self) -> Result<String>;
    fn send_string(&self, s: &String) -> Result<()>;
}

impl StringSocket for UdpSocket {
    fn recv_string(&self) -> Result<String> {
        let mut buf = [0; 128];
        let amt = self.recv(&mut buf)?;
        let msg = String::from_utf8(buf[..amt].to_vec())?;
        Ok(msg)
    }

    fn send_string(&self, msg: &String) -> Result<()> {
        self.send(msg.as_bytes())?;
        Ok(())
    }
}

impl AddrReceiver for UdpSocket {
    fn recv_addr(&self) -> Result<SocketAddr> {
        let mut buf = [0; 128];
        let amt = self.recv(&mut buf)?;
        let other = str::from_utf8(&buf[..amt])?;
        let addr = other.parse()?;
        Ok(addr)
    }
}

fn main() -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("127.0.0.1:34254")?;

    socket.send_string(&"Hello World".to_string())?;

    let addr = socket.recv_addr()?;
    socket.connect(addr)?;
    println!("Connecting to: {}", addr);

    socket.send_string(&format!("Hello {}", addr).to_string())?;

    let msg = socket.recv_string()?;
    println!("Message received: {}", msg);

    socket.send_string(&format!("Goodbye {}", addr).to_string())?;

    let msg = socket.recv_string()?;
    println!("Message received: {}", msg);

    Ok(())
}
