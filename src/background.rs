use bevy::prelude::*;

use crate::{assets::NovelAssets, InitialLoad, ScaleFactor, TARGET_HEIGHT};

#[derive(Resource, Clone)]
pub enum Background {
    Color(Color),
    Image(String),
}

impl Background {
    pub fn is_color(&self) -> bool {
        match self {
            Background::Color(_) => true,
            Background::Image(_) => false,
        }
    }

    pub fn is_image(&self) -> bool {
        match self {
            Background::Color(_) => false,
            Background::Image(_) => true,
        }
    }
}

#[derive(Component, Default)]
pub struct BackgroundComponent {
    pub image_size: Option<Vec2>,
}

impl Default for Background {
    fn default() -> Self {
        Self::Image("testbg".to_string())
    }
}

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Background>()
            .add_systems(Startup, init_background)
            .add_systems(Update, background_manager.run_if(
                in_state(InitialLoad::Done)
            ));
    }
}

pub fn init_background(
    mut commands: Commands,
) {
    commands.spawn((
        BackgroundComponent::default(),
        Sprite::default(),
        TransformBundle::default(),
        VisibilityBundle::default(),
    ));
}

pub fn background_manager(
    mut commands: Commands,

    mut q_background: Query<(Entity, &mut BackgroundComponent)>,
    mut q_visibility: Query<&mut Visibility>,
    mut q_sprite: Query<&mut Sprite>,

    mut clear_color: ResMut<ClearColor>,
    scale: Res<ScaleFactor>,
    background_res: Res<Background>,
    assets: Res<NovelAssets>,
    images: Res<Assets<Image>>,
) {
    if background_res.is_changed() {
        let (background, mut component) = q_background.single_mut();
        let mut vis = q_visibility.get_mut(background).unwrap();

        match background_res.as_ref() {
            Background::Color(color) => {
                clear_color.0 = *color;
                component.image_size = None;
                vis.set_if_neq(Visibility::Hidden);
            },
            Background::Image(name) => {
                if let Some(handle) = assets.images.get(name) {
                    commands.entity(background).insert(handle.to_owned());

                    let img = images.get(handle).unwrap();
                    component.image_size = Some(img.size_f32());
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

    if (scale.is_changed() || background_res.is_changed()) && background_res.is_image()  {
        let (background, component) = q_background.single();
        if let Some(size) = component.image_size {
            let mut sprite = q_sprite.get_mut(background).unwrap();
            let norm_scale = (size.y / TARGET_HEIGHT) * scale.0;
            let sprite_size = size * norm_scale;

            sprite.custom_size = Some(sprite_size);
        }
    }
}