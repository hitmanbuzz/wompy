use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

const IP_ADDR: &str = "127.0.0.1:4096";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let new_stream = tokio::net::TcpStream::connect(IP_ADDR).await;
    match new_stream {
        Ok(mut stream) => {
            let stdin = BufReader::new(tokio::io::stdin());
            let mut lines = stdin.lines();

            loop {
                eprint!(">> ");

                match lines.next_line().await {
                    Ok(Some(line)) => {
                        if line.is_empty() {
                            continue;
                        }

                        let msg = format!("{}\n", line);
                        stream.write_all(msg.as_bytes()).await?;
                        stream.flush().await?;
                    }
                    Ok(None) => break,
                    Err(e) => return Err(e.into()),
                }
            }
        }
        Err(e) => {
            tracing::error!("failed to connect ip {}: {}", IP_ADDR, e);
        }
    }

    Ok(())
}
