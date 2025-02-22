use async_std::net::UdpSocket;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080").await?;
    let mut buf = vec![0u8; 1024];

    loop {
        let (n, peer) = socket.recv_from(&mut buf).await?;
        socket.send_to(&buf[..n], &peer).await?;
    }
}
