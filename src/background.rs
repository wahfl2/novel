use bevy::prelude::*;

#[derive(Resource)]
pub enum Background {
    Color(Color),
    Image,
}

impl Default for Background {
    fn default() -> Self {
        Self::Color(Color::BLACK)
    }
}

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<Background>()
            .add_systems(Update, background_manager);
    }
}

pub fn init_background(
    mut commands: Commands,
) {
    
}

pub fn background_manager(
    mut clear_color: ResMut<ClearColor>,
    background_res: Res<Background>,
) {
    if background_res.is_changed() {
        match background_res.into_inner() {
            Background::Color(color) => {
                clear_color.0 = *color;
            },
            Background::Image => todo!(),
        }
    }
}