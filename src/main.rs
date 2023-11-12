use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server listening on 127.0.0.1:8080");

    loop {
        match listener.accept().await {
            Ok((socket, _)) => {
                tokio::spawn(handle_client(socket));
            }
            Err(e) => eprintln!("Error accepting client: {}", e),
        }
    }
}

async fn handle_client(mut socket: TcpStream) {
    let mut buf = [0u8; 1024];

    loop {
        match socket.read(&mut buf).await {
            Ok(0) => {
                // Connection closed
                break;
            }
            Ok(n) => {
                // Echo the received data back to the client
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("Error writing to socket: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error reading from socket: {}", e);
                break;
            }
        }
    }
}
