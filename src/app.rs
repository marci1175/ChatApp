/// We derive Deserialize/Serialize so we can persist app state on shutdown.
use egui::{Color32, Align};
use egui::color_picker::Alpha;
mod webcom;
use webcom::TcpClient;
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    #[serde(skip)]
    label: String,
    #[serde(skip)]
    tcpc: Option<TcpClient>,
    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
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
            // Example stuff:
            label: "".to_owned(),
            tcpc: None,
            value: 2.7,
            font_size: 18.0,
            color: Color32::from_rgb(255, 255, 255),
            status: String::from("Waiting for user"),
            status_color: Color32::from_rgb(128, 128, 128),
            msg_font_size: 18.0,
            msg_color: Color32::from_rgb(255, 255, 255),
            settings_is_open : false,
            connection_is_open : false,
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
        if let Some(tcpc) = &mut self.tcpc{
            let incoming_msg : String = TcpClient::listen_for_msg(tcpc);
            if incoming_msg.trim().len() > 0{
                self.messages.push(incoming_msg);
                ctx.request_repaint();
            }
            //ctx.request_repaint();
            
        }
        if self.settings_is_open {
            egui::Window::new("Settings")
                .open(&mut self.settings_is_open)
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
            ui.with_layout(egui::Layout::left_to_right(Align::Min), |ui|{
                if ui.button("Connect").clicked(){
                    self.connection_is_open = true;
                }
    
                if ui.button("Settings").clicked(){
                    self.settings_is_open = true;
                }
                });    
            


        });
        egui::CentralPanel::default().show(ctx, |ui| {
                egui::ScrollArea::vertical().id_source("msg_sarea").show(ui, |ui| {
                //add messages here
                    for i in self.messages.iter() {
                        ui.label(egui::RichText::new(i).color(self.msg_color).size(self.msg_font_size));
                    }
                    ctx.request_repaint();

                });
                
        });
        
        egui::TopBottomPanel::bottom("texts").show(ctx, |ui| {
            ui.add_space(12.0);
            ui.with_layout(egui::Layout::left_to_right(Align::Min), |ui|{
                if ui.button("Send message").clicked(){
                    //format the text which is to be sent
                    if let Some(tcpc) = &mut self.tcpc{
                        tcpc.send_message(self.label.clone(), self.username.clone()).expect("Couldnt send msg");
                    }
                    //TcpClient::sendmessage(&mut self.tcpclient ,self.label.clone()).expect("Couldnt send message");
                    self.label.clear();
                };
            });
            ui.add_space(12.0);
            ui.allocate_ui(egui::vec2(ui.available_width(), 125.0), |ui|{
                egui::ScrollArea::vertical().id_source("text_sarea").show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                    ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut self.label)
                        .text_color(self.color)
                        .font(egui::FontId::proportional(self.font_size)));
                });
        
            }); 
            ui.add_space(5.0);
            
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
