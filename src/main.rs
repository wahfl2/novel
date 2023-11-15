#![feature(variant_count)]

use assets::AssetsPlugin;
use background::BackgroundPlugin;
use bevy::{prelude::*, window::WindowResized};
use bevy_egui::EguiPlugin;
use callback::register_callbacks;
use ui::NovelUiPlugin;

pub mod ui;
pub mod util;
pub mod background;
pub mod assets;
pub mod callback;

pub const TARGET_HEIGHT: f32 = 1080.0;

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

#[derive(Resource, Deref, DerefMut, Clone, Copy)]
pub struct ScaleFactor(f32);

impl Default for ScaleFactor {
    fn default() -> Self { Self(1.0) }
}

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_state::<InitialLoad>()
            .init_resource::<ScaleFactor>()
            .add_plugins((
                DefaultPlugins, 
                EguiPlugin,
                AssetsPlugin,
                BackgroundPlugin,
                NovelUiPlugin,
            ))
            .add_systems(Startup, (setup, register_callbacks))
            .add_systems(First, react_window_resize);
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn react_window_resize(
    mut resize: EventReader<WindowResized>,
    mut scale: ResMut<ScaleFactor>,
) {
    if let Some(window) = resize.read().last() {
        let rounded_height = (window.height / 2.0).round() * 2.0;
        **scale = rounded_height / TARGET_HEIGHT;
    }
}