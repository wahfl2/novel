use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use menu::main_menu::MainMenu;

pub mod menu;

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
        app.add_plugins(DefaultPlugins)
            .add_plugins(EguiPlugin)
            .add_state::<AppState>();
            // .add_systems(Update, menu);
    }
}