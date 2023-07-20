/// We derive Deserialize/Serialize so we can persist app state on shutdown.
use egui::{Color32, Align};

//use for win bindings
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_RETURN, VK_SHIFT, VK_CONTROL};
//use for scroll wheel interaction cuz shit crates
use egui::color_picker::Alpha;
mod webcom;
use webcom::TcpClient;
mod etc;
use etc::emojiui;
use rand::{Rng, rngs::ThreadRng};
use std::thread;
use std::time::Duration;
use win32_notification::NotificationBuilder;
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state

pub struct TemplateApp {
    //generate random
    #[serde(skip)]
    msg_len_already_backed: bool,
    #[serde(skip)]
    has_focus: bool,
    #[serde(skip)]
    back_up_messages_num: usize,
    #[serde(skip)]
    ml_is_enabled: bool,
    #[serde(skip)]
    emojiui_is_open: bool,
    #[serde(skip)]
    random_generated: bool,
    #[serde(skip)]
    randomeng: ThreadRng,
    //used for interactive emoji button
    random_emoji: String,
    emoji: Vec<String>,
    #[serde(skip)]
    //inputted text
    label: String,
    #[serde(skip)]
    tcpc: Option<TcpClient>,
    // this how you opt-out of serialization of a member
    #[serde(skip)]
    //currently connected users
    user_counter: String,
    //settings
    font_size : f32,
    color: Color32,
    #[serde(skip)]
    status: String,
    #[serde(skip)]
    status_color: Color32,

    msg_font_size : f32,
    msg_color: Color32,
    //settings menu
    #[serde(skip)]
    settings_is_open : bool,
    #[serde(skip)]
    connection_is_open : bool,

    #[serde(skip)]
    messages: Vec<String>,

    #[serde(skip)]
    username: String,
    #[serde(skip)]
    ip: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            has_focus: false,
            back_up_messages_num: 0,
            ml_is_enabled: false,
            msg_len_already_backed: false,
            emojiui_is_open: false,
            random_generated: false,
            randomeng: rand::thread_rng(),
            emoji: vec!["😐","😍","😉","😈","😇","😆","😅","😄","😃","😂","😁","😀","✡"].into_iter().map(str::to_owned).collect::<Vec<_>>(),
            random_emoji: "😐".to_owned(),
            label: "".to_owned(),
            tcpc: None,
            user_counter: "0".to_string(),
            font_size: 18.0,
            color: Color32::from_rgb(255, 255, 255),
            status: String::from("Waiting for user"),
            status_color: Color32::from_rgb(128, 128, 128),
            msg_font_size: 18.0,
            msg_color: Color32::from_rgb(255, 255, 255),
            settings_is_open : false,
            connection_is_open : true,
            messages: Vec::new(),
            username: String::new(),
            ip: String::from("127.0.0.1:6000"),
        }
    }
}
impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}
impl eframe::App for TemplateApp {
    
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }


    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //check if focused
        self.has_focus = ctx.input(|i| i.focused);
        //check the messages.len() when the app lost focus
        if !self.has_focus && !self.msg_len_already_backed{
            self.back_up_messages_num = self.messages.len();
            self.msg_len_already_backed = true;
        }
        if self.has_focus{
            self.msg_len_already_backed = false;
        }
        //is backed up num is small than messages.len with 5 play notif and add 5 to backup
        if  (self.back_up_messages_num + 5) < self.messages.len(){
            //reset thread_is_running value
            self.back_up_messages_num += 5;
                std::thread::spawn(||{
                    let notification = NotificationBuilder::new()
                        .title_text("SzéChat")
                        .info_text("Unread messages")
                        .build()
                        .expect("Could not create notification");
    
                        notification.show().expect("Failed to show notification");
                    thread::sleep(Duration::from_secs(5));
                    notification
                        .delete()
                        .expect("Failed to delete notification");
                });
        }
        if let Some(tcpc) = &mut self.tcpc{
            let incoming_msg : String = TcpClient::listen_for_msg(tcpc);
            if incoming_msg.trim().len() > 0{
                let raw_msg: Vec<String> = incoming_msg.split('\n').map(|s| s.to_string()).collect();
                self.messages.push(raw_msg[0].clone());
                if raw_msg.len() > 1 {
                    self.user_counter = raw_msg[1].clone();
                }
                else {
                    self.user_counter = "Use the latest server else the user counter wont work!".to_owned();
                }
                ctx.request_repaint();
            }
            //ctx.request_repaint();
            
        }
        //scroll delta = zoom delta because im pressing ctrl it counts az zoom
        let scroll_delta: f32 = ctx.input(|state: &egui::InputState| state.zoom_delta());
        
        
        let ctrlimput = unsafe {
            GetAsyncKeyState(VK_CONTROL as i32)
        };
        let ctrlis_pressed = (ctrlimput as u16 & 0x8000) != 0;
        //listen if ENTER key is pressed so we can send the message, except when r or l shift is pressed
        let enterinp = unsafe{
            GetAsyncKeyState(VK_RETURN as i32)
        };
        let shiftinp = unsafe{
            GetAsyncKeyState(VK_SHIFT as i32)
        };
        
        let sis_pressed = (shiftinp as u16 & 0x8000) != 0;
        let eis_pressed = (enterinp as u16 & 0x8000) != 0;
        
        if eis_pressed && !sis_pressed {
            if let Some(tcpc) = &mut self.tcpc{
                tcpc.send_message(self.label.clone(), self.username.clone()).expect("Couldnt send msg");
            }
            self.label.clear();
        }
        
        if self.settings_is_open {
            egui::Window::new("Settings")
                .open(&mut self.settings_is_open)
                .resizable(false)
                .show(ctx, |ui| {
                    egui::Grid::new("settings_grid").num_columns(2).show(ui, |ui| {
                        ui.label("Text editor");
                        ui.group(|ui|{
                        ui.label("Text size");
                        ui.add(egui::Slider::new(&mut self.font_size, 1.0..=100.0));
                        ui.label("Text color");
                        egui::color_picker::color_picker_color32(ui, &mut self.color, Alpha::Opaque);});
                        ui.end_row();
                        ui.label("Messages");
                        ui.group(|ui|{
                        ui.label("Text size");
                        ui.add(egui::Slider::new(&mut self.msg_font_size, 1.0..=100.0));
                        ui.label("Text color");
                        egui::color_picker::color_picker_color32(ui, &mut self.msg_color, Alpha::Opaque);});
                    });
                });
        }
        if self.connection_is_open{
            egui::Window::new("Connection")
                .open(&mut self.connection_is_open)
                .auto_sized()
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.label("Enter a username");
                    ui.text_edit_singleline(&mut self.username);
                    ui.label("Enter the ip you want to connect to :");
                    ui.text_edit_singleline(&mut self.ip);
                    if ui.button("Connect").clicked(){ 
                       
                        if let Some(_) = &self.tcpc {
                            self.status = "Connection already established".to_owned();
                            ctx.request_repaint();
                        }
                        else if self.username.trim().is_empty(){
                            self.status = "You didnt enter a username!".to_owned();
                            self.status_color = Color32::from_rgb(255, 0, 0);
                            ctx.request_repaint();
                        }
                        else {
                            self.status = "Connecting. . .".to_owned();
                            self.status_color = Color32::from_rgb(127, 250, 133);
                            ctx.request_repaint();
                            match TcpClient::new(&self.ip) {
                                Ok(tcpc) => {
                                    self.tcpc = Some(tcpc);
                                    self.ml_is_enabled = true;
                                    self.status = "Connected".to_owned();
                                    self.status_color = Color32::from_rgb(0, 255, 0);
                                    self.messages.clear();
                                    ctx.request_repaint();
                                },
                                Err(_) => {
                                    self.status = "Invalid ip adress or port".to_owned();
                                    self.status_color = Color32::from_rgb(255, 0, 0);
                                    ctx.request_repaint();
                                },

                            };
                            
                        }
                        
                    }
                    if ui.button("Disconnect").clicked(){
                        //dsiconnect
                        if let Some(tcpc) = &self.tcpc {
                            self.status = "Disconecting. . .".to_owned();
                            self.status_color = Color32::from_rgb(245, 66, 93);
                            ctx.request_repaint();
                            TcpClient::shutdown(tcpc).expect("Couldnt shutdown");
                            self.tcpc = None;
                            self.ml_is_enabled = false;
                            self.status = "Disconected".to_owned();
                            self.status_color = Color32::from_rgb(255, 0, 0);
                            ctx.request_repaint();
                        }
                        else{
                            self.status = "Connection doesnt exist".to_owned();
                            ctx.request_repaint();
                        }
                    }
                    ui.colored_label(self.status_color, &mut self.status)
                });
        }
        

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui|{
               
                if ui.button("Connect").clicked(){
                    self.connection_is_open = true;
                }
    
                if ui.button("Settings").clicked(){
                    self.settings_is_open = true;
                }
                if !self.ml_is_enabled {
                    ui.label(egui::RichText::from("Connect to a chat server to write messages!").color(egui::Color32::from_rgb(255, 0, 0)));
                }
                else {
                    let connected_users = format!("Connected users: {}", self.user_counter);
                    ui.label(connected_users);
                }
                
                });   
            


        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.allocate_ui(egui::vec2(ui.available_width(), ui.available_height() - 180.0), |ui|{
                egui::ScrollArea::vertical().id_source("msg_sarea").stick_to_bottom(true).show(ui, |ui| {
                //add messages here
                    for i in self.messages.iter() {
                        let msglabel = ui.label(egui::RichText::new(i).color(self.msg_color).size(self.msg_font_size));
                        if msglabel.hovered() && ctrlis_pressed{
                            if scroll_delta < 1.0 {
                                self.msg_font_size -= 5.0;
                            }
                            if scroll_delta > 1.0 {
                                self.msg_font_size += 5.0;
                            }
                        }
                        ui.separator();
                    }
                    ctx.request_repaint();

                });
            });
        });
        egui::TopBottomPanel::bottom("texteditor").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.allocate_ui(egui::vec2(ui.available_width(), 125.0), |ui|{
                egui::ScrollArea::vertical().id_source("text_sarea").stick_to_bottom(true).show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                    ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut self.label)
                        .text_color(self.color)
                        .interactive(self.ml_is_enabled)
                        .font(egui::FontId::proportional(self.font_size)));
                    });
                }); 
            ui.add_space(5.0);
            });    
        });
        egui::TopBottomPanel::bottom("textmenu").show(ctx, |ui| {
            if self.emojiui_is_open{
                let emoji = emojiui(ctx, _frame);
                self.label += &emoji;
            }
            ui.allocate_ui(egui::vec2(ui.available_width(), 40.0), |ui|{
                ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui|{
                    if ui.button("Send message").clicked(){
                        //format the text which is to be sent
                        if let Some(tcpc) = &mut self.tcpc{
                            tcpc.send_message(self.label.clone(), self.username.clone()).expect("Couldnt send msg");
                        }
                        self.label.clear();
                    };
                    //emoji icon, button logic
                    let uibutt = ui.button(egui::RichText::from(&self.random_emoji).size(20.0));
                    if uibutt.hovered(){
                        if !self.random_generated {
                            let random_number = self.randomeng.gen_range(0..=self.emoji.len() - 1);
                            self.random_emoji = self.emoji[random_number].clone();
                            self.random_generated = true;
                        }
                    }
                    else {
                        //check if button has been unhovered, reset variable
                        self.random_generated = false;
                    }
                    if uibutt.clicked(){
                        if self.emojiui_is_open {
                            self.emojiui_is_open = false;
                        }
                        else {
                            self.emojiui_is_open = true;
                        }
                        
                    }
    
                });
            });
        });
        
        
        
        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
