use bevy::prelude::*;

use crate::{assets::NovelAssets, InitialLoad};

#[derive(Resource)]
pub enum Background {
    Color(Color),
    Image(String),
}

#[derive(Component)]
pub struct BackgroundComponent;

impl Default for Background {
    fn default() -> Self {
        Self::Image("testbg.png".to_string())
    }
}

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Background>()
            .add_systems(Startup, init_background)
            .add_systems(Update, background_manager.run_if(in_state(InitialLoad::Done)));
    }
}

pub fn init_background(
    mut commands: Commands,
) {
    commands.spawn((
        BackgroundComponent,
        TransformBundle::default(),
        VisibilityBundle::default(),
    ));
}

pub fn background_manager(
    mut commands: Commands,

    q_background: Query<Entity, With<BackgroundComponent>>,
    mut q_visibility: Query<&mut Visibility>,

    mut clear_color: ResMut<ClearColor>,
    background_res: Res<Background>,
    assets: Res<NovelAssets>,
) {
    if background_res.is_changed() {
        let background = q_background.single();
        let mut vis = q_visibility.get_mut(background).unwrap();

        match background_res.into_inner() {
            Background::Color(color) => {
                clear_color.0 = *color;
                vis.set_if_neq(Visibility::Hidden);
            },
            Background::Image(name) => {
                println!("{:?}", assets.images);
                if let Some(img) = assets.images.get(name) {
                    commands.entity(background).insert(img.to_owned());
                } else {
                    eprintln!(
                        "Error: Image '{name}' could not be found. {}",
                        "The background may be displayed incorrectly."
                    );
                }

                vis.set_if_neq(Visibility::Visible);
            },
        }
    }
}