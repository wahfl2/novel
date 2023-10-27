use bevy::prelude::ResMut;
use bevy_egui::{egui::Context, EguiContexts};

use self::{main_menu::MainMenu, layer_stack::UiLayerStack};

pub mod main_menu;
pub mod layer_stack;
pub mod layer;

pub fn ui_renderer(
    mut contexts: EguiContexts,
    mut layer_stack: ResMut<UiLayerStack>,
) {
    let ctx = contexts.ctx_mut();
    
    for layer in layer_stack.top_down() {
        layer.update(ctx)
    }
}