use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    
    let mut buffer = [0u8; 1024];
    let n = stream.read(&mut buffer).await?;
    let message = String::from_utf8_lossy(&buffer[..n]);
    println!("接收到的數據：{}", message);
    
    let data = "Hello, server!";
    stream.write_all(data.as_bytes()).await?;
    
    Ok(())
}

