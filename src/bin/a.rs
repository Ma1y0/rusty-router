use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0u8; 1024];

            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Failed to read from socket, err: {:?}", e);
                        return;
                    }
                };

                println!("{:?}", String::from_utf8(buf[..n].to_vec()).unwrap());

                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("Failed to write socket err: {:?}", e);
                    return;
                }
            }
        });
    }
}
