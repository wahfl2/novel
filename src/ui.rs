use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui::{FontId, self}, EguiSettings};

use crate::{AppState, ScaleFactor};

use self::{layer_stack::UiLayerStack, main_menu::add_main_menu};

pub mod main_menu;
pub mod layer_stack;
pub mod layer;

pub struct NovelUiPlugin;

impl Plugin for NovelUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiLayerStack>()
            .add_systems(Startup, set_default_style)
            .add_systems(PreUpdate, react_rescale)
            .add_systems(Update, ui_renderer)
            .add_systems(OnEnter(AppState::MainMenu), add_main_menu);
    }
}

pub fn ui_renderer(
    mut commands: Commands,
    mut contexts: EguiContexts,
    layer_stack: Res<UiLayerStack>, 
) {
    let ctx = contexts.ctx_mut();

    for layer in layer_stack.last_opaque_up() {
        layer.update(&mut commands, ctx);
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

pub fn react_rescale(
    scale: Res<ScaleFactor>,
    mut egui_settings: ResMut<EguiSettings>,
) {
    if scale.is_changed() {
        egui_settings.scale_factor = **scale as f64;
    }
}