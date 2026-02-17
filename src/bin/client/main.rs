use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

const IP_ADDR: &str = "127.0.0.1:4096";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Clear Screen
    eprint!("\x1B[2J\x1B[1;1H");

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let mut user_name = String::new();
    user_name.clear();

    while user_name.is_empty() {
        eprint!("Enter Username: ");
        std::io::stdin().read_line(&mut user_name)?;
        user_name = user_name.trim().to_string();
        eprint!("\x1B[2J\x1B[1;1H");
    }
    
    let new_stream = tokio::net::TcpStream::connect(IP_ADDR).await;
    match new_stream {
        Ok(mut stream) => {
            let msg_stdin = BufReader::new(tokio::io::stdin());
            let mut lines = msg_stdin.lines();
            stream.writable().await?;

            // send the username to the server
            user_name = format!("{}\n", user_name);
            stream.write_all(user_name.as_bytes()).await?;

            loop {
                eprint!("{} (YOU) >> ", user_name.trim());

                match lines.next_line().await {
                    Ok(Some(line)) => {
                        if line.is_empty() {
                            continue;
                        }

                        // send msg chat to the server
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
