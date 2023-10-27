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

    pub fn push(&mut self, layer: UiLayer) {
        self.stack.push(layer);
    }

    pub fn pop(&mut self) -> Option<UiLayer> {
        self.stack.pop()
    }

    pub fn top_down(&self) -> impl Iterator<Item = &UiLayer> {
        self.stack.iter()
            .rev()
            .take_while_inclusive(|&layer| !layer.opaque())
    }

    pub fn last_opaque_up(&self) -> impl Iterator<Item = &UiLayer> {
        self.top_down().collect::<Vec<_>>().into_iter().rev()
    }
}

