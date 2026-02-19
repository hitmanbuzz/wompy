use eframe::egui;
use tokio::{io::AsyncWriteExt, sync::mpsc};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

const IP_ADDR: &str = "127.0.0.1:4096";
const GOOD_SYM: &str = "✅";
const BAD_SYM: &str = "❌";

struct ChatApp {
    username: String,
    groupname: String,
    is_connected: bool,
    status_color: egui::Color32,
    status_sym: String,

    stream: Option<tokio::net::TcpStream>,

    tx: mpsc::Sender<Option<tokio::net::TcpStream>>,
    rx: mpsc::Receiver<Option<tokio::net::TcpStream>>,
    runtime: tokio::runtime::Handle,

    user_msg: String,
    user_all_msg: Vec<String>,
}

impl ChatApp {
    fn new(runtime: tokio::runtime::Handle) -> Self {
        let (tx, rx) = mpsc::channel(1);

        Self {
            username: String::new(),
            groupname: String::new(),
            is_connected: false,
            status_color: egui::Color32::RED,
            status_sym: BAD_SYM.to_string(),
            stream: None,
            tx,
            rx,
            runtime,
            user_msg: String::new(),
            user_all_msg: Vec::new(),
        }
    }
}

impl eframe::App for ChatApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Ok(result) = self.rx.try_recv() {
            match result {
                Some(stream) => {
                    self.stream = Some(stream);
                    self.status_sym = GOOD_SYM.to_string();
                    self.status_color = egui::Color32::GREEN;
                    self.is_connected = true;
                }
                None => {
                    self.status_sym = BAD_SYM.to_string();
                    self.status_color = egui::Color32::RED;
                }
            }
        }

        egui::TopBottomPanel::top("top_panel")
            .exact_height(50.0)
            .resizable(false)
            .show(ctx, |tui| {
                tui.add_space(8.0);
                tui.horizontal(|hui| {
                    let username_label = hui.add(egui::Label::new(
                        egui::RichText::new("Username:")
                            .size(16.0)
                            .strong()
                            .line_height(Some(6.0)) // some dumb workaround
                            .color(egui::Color32::LIGHT_BLUE),
                    ));
                    hui.add(
                        egui::TextEdit::singleline(&mut self.username)
                            .char_limit(18)
                            .vertical_align(egui::Align::Center)
                            .hint_text("type username")
                            .min_size(egui::Vec2::new(100.0, 30.0))
                            .desired_width(200.0)
                            .font(egui::FontId::new(16.0, egui::FontFamily::Proportional)),
                    )
                    .labelled_by(username_label.id);

                    let groupname_label = hui.add(egui::Label::new(
                        egui::RichText::new("Group Name:")
                            .size(16.0)
                            .strong()
                            .color(egui::Color32::LIGHT_BLUE),
                    ));
                    hui.add(
                        egui::TextEdit::singleline(&mut self.groupname)
                            .char_limit(18)
                            .vertical_align(egui::Align::Center)
                            .hint_text("type groupname")
                            .min_size(egui::Vec2::new(100.0, 30.0))
                            .desired_width(200.0)
                            .font(egui::FontId::new(16.0, egui::FontFamily::Proportional)),
                    )
                    .labelled_by(groupname_label.id);

                    let join_btn = hui.add_sized(
                        [60.0, 30.0],
                        egui::Button::new(egui::RichText::new("Join").strong()),
                    );

                    // when the submit btn is clicked, join the server group with the provided username and groupname
                    if join_btn.clicked()
                        && !self.username.is_empty()
                        && !self.groupname.is_empty()
                        && !self.is_connected
                    {
                        let username = self.username.clone();
                        let groupname = self.groupname.clone();
                        let tx = self.tx.clone();
                        let ctx = ctx.clone();

                        self.runtime.spawn(async move {
                            let result = Some(connect_server(IP_ADDR).await);
                            match result {
                                Some(mut stream) => {
                                    let init_success = send_init_data(&username, &groupname, &mut stream).await;
                                    
                                    if init_success {
                                        let _ = tx.send(Some(stream)).await;
                                    } else {
                                        tracing::error!("failed to send username and groupname to the server");   
                                    }
                                },
                                None => todo!(),
                            }
                            ctx.request_repaint();
                        });
                    }

                    hui.add_space(15.0);

                    let status_label = hui.add(egui::Label::new(
                        egui::RichText::new("Status:")
                            .size(16.0)
                            .strong()
                            .color(egui::Color32::LIGHT_BLUE),
                    ));
                    hui.add(egui::Label::new(
                        egui::RichText::new(self.status_sym.as_str())
                            .color(self.status_color)
                            .size(24.0),
                    ))
                    .labelled_by(status_label.id);
                });
            });

        // this will show all the current users in the group
        egui::SidePanel::left("left_panel")
            .exact_width(150.0)
            .resizable(false)
            .show(ctx, |lui| {
                lui.add_sized(
                    [lui.available_width(), 20.0],
                    egui::Label::new(egui::RichText::new("USERS").strong().underline().size(24.0)),
                );

                // put all the users in this section
                egui::ScrollArea::vertical().show(lui, |sui| {
                    sui.with_layout(
                        egui::Layout::top_down_justified(egui::Align::Center),
                        |lsui| {
                            // TODO
                        },
                    )
                });
            });

        // this is where user message field box and send button is placed
        egui::TopBottomPanel::bottom("bottom_panel")
            .default_height(30.0)
            .min_height(30.0)
            .max_height(100.0)
            .resizable(true)
            .show(ctx, |bui| {
                bui.add_space(1.5);
                bui.horizontal(|hui| {
                    let size = egui::vec2(hui.available_width() - 100.0, hui.available_height());

                    hui.add_sized(
                        size,
                        egui::TextEdit::multiline(&mut self.user_msg)
                            .hint_text("type message")
                            .desired_rows(1)
                            .font(egui::FontId::new(16.0, egui::FontFamily::Proportional)),
                    );

                    hui.add_space(5.0);

                    let send_msg_btn = hui.add_sized(
                        [80.0, hui.available_height()],
                        egui::Button::new(egui::RichText::new("Send").strong()),
                    );

                    // send the chat msg to the server group
                    if send_msg_btn.clicked() && !self.user_msg.is_empty() {
                        self.user_all_msg.push(self.user_msg.clone());
                        self.user_msg.clear();
                    }
                });
            });

        // this is where all the messages from all users in the group are shown
        egui::CentralPanel::default().show(ctx, |cui| {
            egui::ScrollArea::vertical().show(cui, |sui| {
                sui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |lsui| {
                    for msg in self.user_all_msg.iter() {
                        lsui.label(egui::RichText::new(msg).size(15.0).strong());
                        lsui.add_space(5.0);
                    }
                });
            });
        });
    }
}

#[tokio::main]
async fn main() -> eframe::Result<()> {
    eprint!("\x1B[2J\x1B[1;1H");

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::ERROR)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let runtime = tokio::runtime::Handle::current();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([800.0, 600.0])
            .with_max_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Chat App",
        options,
        Box::new(|_| Ok(Box::new(ChatApp::new(runtime)))),
    )?;

    Ok(())
}

async fn connect_server(ip_addr: &str) -> tokio::net::TcpStream {
    let new_stream = tokio::net::TcpStream::connect(ip_addr).await;

    match new_stream {
        Ok(stream) => return stream,
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::HostUnreachable => {
                    panic!("Host Unreachable (Not Found)");
                },
                std::io::ErrorKind::AddrNotAvailable => {
                    panic!("Host not found");
                },
                std::io::ErrorKind::ConnectionRefused => {
                    panic!("Got rejected by the host");
                }
                _ => {
                    panic!("Got Error: {}", e);
                }
            }
        },
    }
}

/// Send username and groupname after connecting to the server
async fn send_init_data(
    user_name: &str,
    group_name: &str,
    stream: &mut tokio::net::TcpStream,
) -> bool {
    let username = format!("{}\n", user_name);
    let groupname = format!("{}\n", group_name);

    let (mut user_good, mut group_good) = (false, false);

    if let Ok(_) = stream.write_all(username.as_bytes()).await {
        user_good = true;
    }

    if let Ok(_) = stream.write_all(groupname.as_bytes()).await {
        group_good = true;
    }

    if user_good && group_good {
        tracing::info!(
            "user `{}` has joined the server in group: `{}`",
            username.trim(),
            groupname.trim()
        );
        return true;
    }

    return false;
}
