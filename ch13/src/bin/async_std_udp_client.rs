use async_std::net::UdpSocket;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    let server_addr = "127.0.0.1:8080";
    let msg = b"Hello, world!";
    
    socket.send_to(msg, server_addr).await?;
    
    let mut buf = vec![0u8; 1024];
    let (n, _) = socket.recv_from(&mut buf).await?;
    
    println!("Received: {}", String::from_utf8_lossy(&buf[..n]));
    
    Ok(())
}
