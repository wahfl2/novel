use bevy::{prelude::*, utils::HashMap};
use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::{LoadingStateAppExt, LoadingState}};

use crate::InitialLoad;

#[derive(AssetCollection, Resource)]
pub struct NovelAssets {
    #[asset(path = "images", collection(typed, mapped))]
    pub images: HashMap<String, Handle<Image>>,

    #[asset(path = "sounds", collection(typed, mapped))]
    pub sounds: HashMap<String, Handle<AudioSource>>,
}


pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loading_state(
                LoadingState::new(InitialLoad::Loading)
                    .continue_to_state(InitialLoad::Cleaning)
            )
            .add_collection_to_loading_state::<_, NovelAssets>(InitialLoad::Loading)
            .add_systems(OnEnter(InitialLoad::Cleaning), clean);
    }
}

pub fn clean(
    mut assets: ResMut<NovelAssets>,
    mut state: ResMut<NextState<InitialLoad>>,
) {
    assets.clean_keys();
    state.set(InitialLoad::Done);
}

impl NovelAssets {
    pub fn clean_keys(&mut self) {
        clean_map(&mut self.images);
        clean_map(&mut self.sounds);

        fn clean_map<V>(map: &mut HashMap<String, V>) {
            for (key, image) in map.drain().collect::<Vec<_>>() {
                let clean = key
                    .split_once('/') 
                    .unwrap().1 // Remove prefix, i.e. 'images/'
                    .rsplit_once(".")
                    .unwrap().0; // Remove extension, i.e. '.png'
    
                map.insert(clean.to_string(), image);
            }
        }
    }

    pub fn image(&self, name: &str) -> Handle<Image> {
        self.images
            .get(name)
            .expect("Could not find image {name}.")
            .clone()
    }
}