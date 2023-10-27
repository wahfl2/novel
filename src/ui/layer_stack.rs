use bevy::prelude::Resource;

use crate::util::take_while_incl::MyIterator;

use super::layer::UiLayer;

#[derive(Resource)]
pub struct UiLayerStack {
    pub stack: Vec<UiLayer>
}

impl Default for UiLayerStack {
    fn default() -> Self {
        Self { stack: Vec::new() }
    }
}

impl UiLayerStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn top_down(&self) -> impl Iterator<Item = &UiLayer> {
        self.stack.iter()
            .rev()
            .take_while_inclusive(|&layer| !layer.opaque())
    }
}

