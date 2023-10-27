use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use ui::layer_stack::UiLayerStack;

pub mod ui;
pub mod util;

fn main() {
    App::new()
        .add_plugins(MainPlugin)
        .run();
}

#[derive(States, Clone, Hash, PartialEq, Eq, Debug, Default)]
pub enum AppState {
    #[default]
    Startup,
    MainMenu,
    InGame,
    Cutscene,
}

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiLayerStack>()
            .add_plugins(DefaultPlugins)
            .add_plugins(EguiPlugin)
            .add_state::<AppState>();
            // .add_systems(Update, menu);
    }
}