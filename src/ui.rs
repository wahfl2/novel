use bevy::{prelude::*, window::WindowResized};
use bevy_egui::{EguiContexts, egui::{FontId, self}, EguiSettings};

use crate::AppState;

use self::{layer_stack::UiLayerStack, main_menu::add_main_menu};

pub mod main_menu;
pub mod layer_stack;
pub mod layer;

pub struct NovelUiPlugin;

impl Plugin for NovelUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiLayerStack>()
            .add_systems(Startup, set_default_style)
            .add_systems(First, react_window_resize)
            .add_systems(Update, ui_renderer)
            .add_systems(OnEnter(AppState::MainMenu), add_main_menu);
    }
}

pub fn ui_renderer(
    mut contexts: EguiContexts,
    layer_stack: Res<UiLayerStack>, 
) {
    let ctx = contexts.ctx_mut();

    for layer in layer_stack.last_opaque_up() {
        layer.update(ctx)
    }
}

pub fn set_default_style(
    mut contexts: EguiContexts,
) {
    let ctx = contexts.ctx_mut();

    ctx.style_mut(|style| {
        style.text_styles.insert(
            egui::TextStyle::Button, 
            FontId::proportional(30.0)
        );
    });
}

pub fn react_window_resize(
    mut resize_reader: EventReader<WindowResized>,
    mut egui_settings: ResMut<EguiSettings>,
) {
    if let Some(window) = resize_reader.into_iter().last() {
        const TARGET_SIZE: f32 = 1080.0;
        let window_size = window.height;

        egui_settings.scale_factor = (window_size / TARGET_SIZE) as f64;
    }
}