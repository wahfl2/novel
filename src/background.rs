use bevy::prelude::*;

#[derive(Resource)]
pub enum Background {
    Color(Color),
    Image(String),
}

#[derive(Component)]
pub struct BackgroundComponent;

impl Default for Background {
    fn default() -> Self {
        Self::Color(Color::BLACK)
    }
}

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<Background>()
            .add_systems(Startup, init_background)
            .add_systems(Update, background_manager);
    }
}

pub fn init_background(
    mut commands: Commands,
) {
    commands.spawn((
        TransformBundle::default(),
        VisibilityBundle::default(),
        BackgroundComponent,
    ));
}

pub fn background_manager(
    mut commands: Commands,

    q_background: Query<Entity, With<BackgroundComponent>>,
    mut q_visibility: Query<&mut Visibility>,

    mut clear_color: ResMut<ClearColor>,
    background_res: Res<Background>,
    asset_server: Res<AssetServer>,
) {
    if background_res.is_changed() {
        let background = q_background.single();
        let vis = q_visibility.get_mut(background).unwrap();

        match background_res.into_inner() {
            Background::Color(color) => {
                clear_color.0 = *color;
                set_visibility(vis, Visibility::Hidden);
            },
            Background::Image(path) => {
                let img = asset_server.load::<Image, _>(path);
                commands.entity(background).insert(img);
                set_visibility(vis, Visibility::Visible);
            },
        }
    }

    fn set_visibility(mut vis: Mut<'_, Visibility>, new_visibility: Visibility) {
        if vis.as_ref() == new_visibility {
            *vis = new_visibility;
        }
    }
}