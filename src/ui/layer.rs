use bevy_egui::egui::Context;

use super::main_menu::MainMenu;

pub enum UiLayer {
    MainMenu(MainMenu)
}

impl UiLayer {
    pub fn opaque(&self) -> bool {
        match self {
            UiLayer::MainMenu(_) => true,
        }
    }

    pub fn update(&self, ctx: &Context) {
        match self {
            UiLayer::MainMenu(m) => m.update(ctx),
        }
    }
}