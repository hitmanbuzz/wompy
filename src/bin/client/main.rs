use eframe::egui;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

const IP_ADDR: &str = "127.0.0.1:4096";
const GOOD_SYM: &str = "✅";
const BAD_SYM: &str = "❌";

struct JoinStatus {
    status: bool,
    sym: String,
    color: egui::Color32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([800.0, 600.0])
            .with_max_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    let mut username = String::new();
    let mut groupname = String::new();

    // indicates whether the user join the server or not
    let mut status = JoinStatus {
        status: false,
        sym: BAD_SYM.to_string(),
        color: egui::Color32::RED,
    };

    let mut usr_msgs: Vec<String> = Vec::new();
    let mut usr_msg_field = String::new();

    eframe::run_simple_native("Chat App", options, move |ui, _frame| {
        // this is where we put username and group name for the user
        egui::TopBottomPanel::top("top_panel")
            .exact_height(50.0)
            .resizable(false)
            .show(ui, |tui| {
                tui.add_space(8.0);
                tui.horizontal(|hui| {
                    let username_label = hui.add(egui::Label::new(
                        egui::RichText::new("Username:")
                            .size(16.0)
                            .strong()
                            .line_height(Some(6.0)) // some dumb workaround
                            .color(egui::Color32::LIGHT_BLUE)
                    ));
                    hui.add(
                        egui::TextEdit::singleline(&mut username)
                            .char_limit(18)
                            .vertical_align(egui::Align::Center)
                            .hint_text("type username")
                            .min_size(egui::Vec2::new(100.0, 30.0))
                            .desired_width(200.0)
                            .font(egui::FontId::new(16.0, egui::FontFamily::Proportional))
                    )
                    .labelled_by(username_label.id);

                    let groupname_label = hui.add(egui::Label::new(
                        egui::RichText::new("Group Name:")
                            .size(16.0)
                            .strong()
                            .color(egui::Color32::LIGHT_BLUE)
                    ));
                    hui.add(
                        egui::TextEdit::singleline(&mut groupname)
                            .char_limit(18)
                            .vertical_align(egui::Align::Center)
                            .hint_text("type groupname")
                            .min_size(egui::Vec2::new(100.0, 30.0))
                            .desired_width(200.0)
                            .font(egui::FontId::new(16.0, egui::FontFamily::Proportional))
                    )
                    .labelled_by(groupname_label.id);

                    let submit_btn = hui.add_sized(
                        [60.0, 30.0],
                        egui::Button::new(
                            egui::RichText::new("Join")
                                .strong()
                        )
                    );

                    // when the submit btn is clicked, join the server group with the provided username and groupname
                    if submit_btn.clicked() {
                        
                    }

                    // update the status of the user joining or not to the server
                    match status.status {
                        true => {
                            status.sym = GOOD_SYM.to_string();
                            status.color = egui::Color32::GREEN;
                        },
                        false => {
                            status.sym = BAD_SYM.to_string();
                            status.color = egui::Color32::RED;
                        },
                    }

                    hui.add_space(15.0);

                    let status_label = hui.add(egui::Label::new(
                        egui::RichText::new("Status:")
                            .size(16.0)
                            .strong()
                            .color(egui::Color32::LIGHT_BLUE)
                    ));
                    hui.add(
                        egui::Label::new(
                            egui::RichText::new(status.sym.as_str())
                                .color(status.color)
                                .size(24.0)
                        )
                    ).labelled_by(status_label.id);

                });
            });

        // this will show all the current users in the group
        egui::SidePanel::left("left_panel")
            .exact_width(150.0)
            .resizable(false)
            .show(ui, |lui| {
                lui.label("Left Panel");
        });

        // this is where user message field box and send button is placed
        egui::TopBottomPanel::bottom("bottom_panel")
            .default_height(30.0)
            .min_height(30.0)
            .max_height(100.0)
            .resizable(true)
            .show(ui, |bui|
            {
                bui.add_space(1.5);
                bui.horizontal(|hui| {
                    let size = egui::vec2(hui.available_width() - 100.0, hui.available_height());

                    hui.add_sized(
                        size,
                        egui::TextEdit::multiline(&mut usr_msg_field)
                            .hint_text("type message")
                            .desired_rows(1)
                            .font(egui::FontId::new(16.0, egui::FontFamily::Proportional)),
                    );

                    hui.add_space(5.0);
                
                    let send_msg_btn = hui.add_sized(
                        [80.0, hui.available_height()],
                        egui::Button::new(
                            egui::RichText::new("Send")
                                .strong()
                        )
                    );

                    // send the chat msg to the server group
                    if send_msg_btn.clicked() && !usr_msg_field.is_empty() {
                        usr_msgs.push(usr_msg_field.clone());
                    }
                });
            }
        );

        // this is where all the messages from all users in the group are shown
        egui::CentralPanel::default().show(ui, |cui| {
            egui::ScrollArea::vertical().show(cui, |sui| {
                sui.with_layout(
                    egui::Layout::top_down(egui::Align::LEFT), |lsui| {
                        for msg in usr_msgs.iter() {
                            lsui.label(
                                egui::RichText::new(msg)
                                    .size(15.0)
                                    .strong()
                            );
                            lsui.add_space(5.0);
                        }
                    }
                );
            });
        });
    })
    .unwrap();

    Ok(())
}

#[allow(dead_code)]
async fn main2() -> anyhow::Result<()> {
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
