use assets::AssetsPlugin;
use background::BackgroundPlugin;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use ui::NovelUiPlugin;

pub mod ui;
pub mod util;
pub mod background;
pub mod assets;

fn main() {
    App::new()
        .add_plugins(MainPlugin)
        .run();
}

#[derive(States, Clone, Hash, PartialEq, Eq, Debug, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    Cutscene,
}

#[derive(States, Clone, Hash, PartialEq, Eq, Debug, Default)]
pub enum InitialLoad {
    #[default]
    Loading,
    Cleaning,
    Done,
}

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_state::<InitialLoad>()
            .add_plugins((
                DefaultPlugins, 
                EguiPlugin,
                AssetsPlugin,
                BackgroundPlugin,
                NovelUiPlugin,
            ))
            .add_systems(Startup, setup);
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}