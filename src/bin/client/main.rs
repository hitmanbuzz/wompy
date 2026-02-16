use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

const IP_ADDR: &str = "127.0.0.1:4096";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
    
    let new_stream = tokio::net::TcpStream::connect(IP_ADDR).await;
    match new_stream {
        Ok(mut stream) => {
            let msg_stdin = BufReader::new(tokio::io::stdin());
            let username_stdin = BufReader::new(tokio::io::stdin());
            let groupname_stdin = BufReader::new(tokio::io::stdin());

            let mut username = username_stdin.lines();
            let mut groupname = groupname_stdin.lines();

            let (mut user_good, mut group_good) = (false, false);
            
            loop {
                if user_good && group_good {
                    break;
                }
                
                eprint!("Enter Username: ");
                if let Ok(user_name) = username.next_line().await && !user_good {
                    match user_name {
                        Some(name) => {
                            if name.len() > 0 {
                                let name = format!("{}\n", name);
                                stream.write_all(name.as_bytes()).await?;
                                user_good = true;
                            }
                        },
                        None => {
                            tracing::error!("failed to get next line for username");
                            continue;
                        },
                    }
                }

                eprint!("Enter GroupName: ");
                if let Ok(group_name) = groupname.next_line().await && !group_good {
                    match group_name {
                        Some(g_name) => {
                            if g_name.len() > 0 {
                                let g_name = format!("{}\n", g_name);
                                stream.write_all(g_name.as_bytes()).await?;
                                group_good = true;
                            }
                        },
                        None => {
                            tracing::error!("failed to get next line for groupname");
                            continue;
                        },
                    }
                }
            }
            
            let mut lines = msg_stdin.lines();

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
