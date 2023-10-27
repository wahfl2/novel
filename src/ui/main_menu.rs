use bevy::prelude::ResMut;
use bevy_egui::egui::{Area, Align2, Context, RichText};
use bevy_egui::egui;

use super::layer::UiLayer;
use super::layer_stack::UiLayerStack;

pub struct MainMenu {
    pub area: Area,
    pub labels: Vec<String>,
}

impl MainMenu {
    pub fn new(button_labels: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let area = Area::new("main-menu")
            .movable(false)
            .anchor(Align2::CENTER_BOTTOM, (0.0, -300.0));

        Self {
            area,
            labels: button_labels.into_iter().map(|s| { s.into() }).collect()
        }
    }
    
    pub fn update(&self, ctx: &Context) {
        self.area.show(ctx, |ui| {
            for label in &self.labels {
                ui.allocate_ui_with_layout(
                    egui::vec2(210.0, 0.0), 
                    egui::Layout::centered_and_justified(egui::Direction::LeftToRight), 

                    |ui| {
                        let button = egui::Button::new(label);
                        ui.add(button);
                    }
                );
            }
        });
    }
}

pub fn add_main_menu(mut layer_stack: ResMut<UiLayerStack>) {
    let layer = UiLayer::MainMenu(MainMenu::new([
        "New Game", 
        "Continue", 
        "Options",
        "Quit Game"
    ]));

    layer_stack.push(layer)
}