use std::sync::OnceLock;

use bevy::{prelude::*, ecs::system::SystemId, app::AppExit};

#[repr(usize)]
#[derive(Debug, Clone, Copy)]
pub enum Callback {
    None,
    Quit,
    System(SystemId),
}

impl Callback {
    pub const fn num_variants() -> usize {
        std::mem::variant_count::<Self>()
    }

    pub fn discriminant(&self) -> usize {
        // Copied from https://doc.rust-lang.org/std/mem/fn.discriminant.html
        unsafe { *<*const _>::from(self).cast::<usize>() }
    }
}

pub struct CallbackButton {
    pub label: String,
    pub callback: Callback,
}

impl CallbackButton {
    pub fn new(label: &str, callback: Callback) -> Self { 
        Self { label: label.to_string(), callback }
    }
}

pub trait CommandsCallback {
    fn run_callback(&mut self, callback: Callback);
}

impl CommandsCallback for Commands<'_, '_> {
    fn run_callback(&mut self, callback: Callback) {
        match callback {
            Callback::None => (),
            Callback::System(id) => self.run_system(id),

            callback => self.run_system(
                CALLBACKS.get()
                        .expect("Callbacks not registered!")
                        .get(callback)
            )
        }
    }
}

static CALLBACKS: OnceLock<Callbacks> = OnceLock::new();

#[derive(Debug)]
pub struct Callbacks {
    inner: [Option<SystemId>; Callback::num_variants()]
}

impl Default for Callbacks {
    fn default() -> Self {
        Self { inner: [None; Callback::num_variants()] }
    }
}

impl Callbacks {
    fn insert(&mut self, key: Callback, system: SystemId) {
        self.inner[key.discriminant()] = Some(system);
    }

    pub fn get(&self, key: Callback) -> SystemId {
        self.inner[key.discriminant()].expect(
            &format!("{:?} does not have an associated system. Maybe I need to match it?", key)
        )
    }
}

pub fn register_callbacks(world: &mut World) {
    let mut callbacks = Callbacks::default();

    callbacks.insert(Callback::Quit, world.register_system(s_quit));

    CALLBACKS.set(callbacks).unwrap();
}

pub fn s_quit(mut exit: ResMut<Events<AppExit>>) {
    exit.send(AppExit);
}