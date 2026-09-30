use tokio::net::TcpListener;

// TODO: write an echo server that accepts TCP connections on two listeners, concurrently.
//  Multiple connections (on the same listeners) should be processed concurrently.
//  The received data should be echoed back to the client.
pub async fn echoes_v1(first: TcpListener, second: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let first_c = first.accept().await?;
        tokio::spawn(async {
            let (mut stream, _) = first_c;
            let (mut r, mut w) = stream.split();
            tokio::io::copy(&mut r, &mut w)
                .await
                .expect("copy should not fail");
        });

        let second_c = second.accept().await?;
        tokio::spawn(async {
            let (mut stream, _) = second_c;
            let (mut r, mut w) = stream.split();
            tokio::io::copy(&mut r, &mut w)
                .await
                .expect("copy should not fail");
        });
    }
}

pub async fn echoes(first: TcpListener, second: TcpListener) -> Result<(), anyhow::Error> {
    tokio::spawn(async {
        echo(first).await.expect("don't fail echo");
    });

    tokio::spawn(async {
        echo(second).await.expect("don't fail echo");
    });

    Ok(())
}

pub async fn echo(l: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let c = l.accept().await?;
        tokio::spawn(async {
            let (mut stream, _) = c;
            let (mut r, mut w) = stream.split();
            tokio::io::copy(&mut r, &mut w)
                .await
                .expect("copy should not fail");
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::panic;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::task::JoinSet;

    async fn bind_random() -> (TcpListener, SocketAddr) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        (listener, addr)
    }

    #[tokio::test]
    async fn test_echo() {
        let (first_listener, first_addr) = bind_random().await;
        let (second_listener, second_addr) = bind_random().await;
        tokio::spawn(echoes(first_listener, second_listener));

        let requests = vec!["hello", "world", "foo", "bar"];
        let mut join_set = JoinSet::new();

        for request in requests.clone() {
            for addr in [first_addr, second_addr] {
                join_set.spawn(async move {
                    let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
                    let (mut reader, mut writer) = socket.split();

                    // Send the request
                    writer.write_all(request.as_bytes()).await.unwrap();
                    // Close the write side of the socket
                    writer.shutdown().await.unwrap();

                    // Read the response
                    let mut buf = Vec::with_capacity(request.len());
                    reader.read_to_end(&mut buf).await.unwrap();
                    assert_eq!(&buf, request.as_bytes());
                });
            }
        }

        while let Some(outcome) = join_set.join_next().await {
            if let Err(e) = outcome {
                if let Ok(reason) = e.try_into_panic() {
                    panic::resume_unwind(reason);
                }
            }
        }
    }
}
