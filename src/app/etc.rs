//ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui|{});
pub fn emojiui(ui : &mut egui::Ui, ctx : &egui::Context, _frame : &eframe::Frame) -> String {
    let mut emoji: String = String::new();
    egui::Window::new("Emojis").fixed_size(egui::vec2(100.0, 175.0)).collapsible(false).resizable(false).show(ctx,|ui|{
        egui::ScrollArea::vertical().id_source("text_sarea").show(ui, |ui| {
        ui.label("Faces");
        ui.group(|ui|{
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui|{
                if ui.button("😃").clicked(){
                    emoji = "😃".to_string();
                };
                if ui.button("😁").clicked(){
                    emoji = "😁".to_string();
                };
                if ui.button("😂").clicked(){
                    emoji = "😂".to_string();
                };
                if ui.button("😄").clicked(){
                    emoji = "😄".to_string();
                };
                if ui.button("😅").clicked(){
                    emoji = "😅".to_string();
                };
                if ui.button("😆").clicked(){
                    emoji = "😆".to_string();
                };
                if ui.button("😉").clicked(){
                    emoji = "😉".to_string();
                };
                if ui.button("😍").clicked(){
                    emoji = "😍".to_string();
                };
                if ui.button("😐").clicked(){
                    emoji = "😐".to_string();
                };
                if ui.button("✡").clicked(){
                    emoji = "✡".to_string();
                };
                ui.end_row();
                if ui.button("😇").clicked(){
                    emoji = "😇".to_string();
                };
                if ui.button("👽").clicked(){
                    emoji = "👽".to_string();
                };
                if ui.button("👾").clicked(){
                    emoji = "👾".to_string();
                };
                if ui.button("💀").clicked(){
                    emoji = "💀".to_string();
                };
                if ui.button("👳").clicked(){
                    emoji = "👳".to_string();
                };
                if ui.button("👴").clicked(){
                    emoji = "👴".to_string();
                };
                if ui.button("👵").clicked(){
                    emoji = "👵".to_string();
                };
                if ui.button("👱").clicked(){
                    emoji = "👱".to_string();
                };
                if ui.button("👲").clicked(){
                    emoji = "👲".to_string();
                };
            });
        });
    });
});
    
return emoji;
}