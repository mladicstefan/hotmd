use std::io;
use tokio::{io::AsyncReadExt, net::TcpListener};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6969").await?;

    let mut buff: Vec<u8> = Vec::new();

    let (mut socket, _) = listener.accept().await?;
    loop {
        let incoming = socket.read(&mut buff).await?;
        println!("{:?}", incoming);
    }
}
