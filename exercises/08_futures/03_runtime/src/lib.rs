// TODO: Implement the `fixed_reply` function. It should accept two `TcpListener` instances,
//  accept connections on both of them concurrently, and always reply to clients by sending
//  the `Display` representation of the `reply` argument as a response.
use std::fmt::Display;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

pub async fn fixed_reply<T>(first: TcpListener, second: TcpListener, reply: T)
where
    // `T` cannot be cloned. How do you share it between the two server tasks?
    T: Display + Send + Sync + 'static,
{
    let shareable_reply = format!("{reply}");

    let first_reply = shareable_reply.clone();
    let first_reply = Box::new(first_reply.into_bytes());
    let first_reply = first_reply.leak();

    let second_reply = shareable_reply.clone();
    let second_reply = Box::new(second_reply.into_bytes());
    let second_reply = second_reply.leak();

    loop {
        let first_c = first.accept().await.expect("hope for the best!");
        tokio::spawn(async {
            let (mut stream, _) = first_c;
            if let Ok(_) = stream.writable().await {
                stream.try_write(first_reply).expect("hope for the best!");
            }
        });

        let second_c = second.accept().await.expect("hope for the best!");
        tokio::spawn(async {
            let (mut stream, _) = second_c;
            if let Ok(_) = stream.writable().await {
                stream.try_write(second_reply).expect("hope for the best!");
            }
        });
    }

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::panic;
    use tokio::io::AsyncReadExt;
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
        let reply = "Yo";
        tokio::spawn(fixed_reply(first_listener, second_listener, reply));

        let mut join_set = JoinSet::new();

        for _ in 0..3 {
            for addr in [first_addr, second_addr] {
                join_set.spawn(async move {
                    let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
                    let (mut reader, _) = socket.split();

                    // Read the response
                    let mut buf = Vec::new();
                    reader.read_to_end(&mut buf).await.unwrap();
                    assert_eq!(&buf, reply.as_bytes());
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
