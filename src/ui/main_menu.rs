use bevy::ecs::system::Commands;
use bevy::ecs::world::World;
use bevy_egui::egui::{Area, Align2, Context};
use bevy_egui::egui;

use crate::callback::{CallbackButton, Callback, CommandsCallback};

use super::layer::UiLayer;
use super::layer_stack::UiLayerStack;

pub struct MainMenu {
    pub area: Area,
    pub buttons: Vec<CallbackButton>,
}

impl MainMenu {
    pub fn new(buttons: impl IntoIterator<Item = CallbackButton>) -> Self {
        let area = Area::new("main-menu")
            .movable(false)
            .anchor(Align2::CENTER_BOTTOM, (0.0, -300.0));

        Self {
            area,
            buttons: buttons.into_iter().collect()
        }
    }
    
    pub fn update(&self, commands: &mut Commands, ctx: &Context) {
        self.area.show(ctx, |ui| {
            for button in &self.buttons {
                ui.allocate_ui_with_layout(
                    egui::vec2(210.0, 0.0), 
                    egui::Layout::centered_and_justified(egui::Direction::LeftToRight), 

                    |ui| {
                        let e_button = egui::Button::new(&button.label);
                        if ui.add(e_button).clicked() {
                            commands.run_callback(button.callback);
                        }
                    }
                );
            }
        });
    }
}

pub fn add_main_menu(world: &mut World) {

    let layer = UiLayer::MainMenu(MainMenu::new([
        CallbackButton::new("New Game", Callback::None), 
        CallbackButton::new("Continue", Callback::None), 
        CallbackButton::new("Options", Callback::None),
        CallbackButton::new("Quit Game", Callback::Quit)
    ]));

    let mut layer_stack = world.resource_mut::<UiLayerStack>();
    layer_stack.push(layer)
}