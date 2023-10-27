use bevy_egui::egui::{Area, Align2, Context};
use bevy_egui::egui;

pub struct MainMenu {
    pub area: Area,
    pub labels: Vec<String>,
}

impl MainMenu {
    pub fn new(button_labels: &[String]) -> Self {
        let area = Area::new("main-menu")
            .movable(false)
            .anchor(Align2::CENTER_CENTER, (0.0, 0.0));

        Self {
            area,
            labels: button_labels.to_owned()
        }
    }
    
    pub fn update(&self, ctx: &Context) {
        self.area.show(ctx, |ui| {
            for label in &self.labels {
                ui.add(egui::Button::new(label));
            }
        });
    }
}