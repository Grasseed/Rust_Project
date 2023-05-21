use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("伺服器已啟動，正在監聽連線...");

    loop {
        let (mut stream, _) = listener.accept().await?;
        println!("有新的連線來自客戶端");

        let mut buffer = [0u8; 1024];
        let n = stream.read(&mut buffer).await?;
        let message = String::from_utf8_lossy(&buffer[..n]);
        println!("接收到的數據：{}", message);

        let response = "Hello, client!";
        stream.write_all(response.as_bytes()).await?;
    }
}
