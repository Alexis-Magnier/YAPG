use godot::prelude::*;

#[allow(unused)]
use terrain_gen_stages;

#[allow(unused)]
use terrain_gen_core;

pub mod render;
pub mod terrain_gen;
pub mod backend;

mod planet;

struct PlanetExtension;

#[gdextension]
unsafe impl ExtensionLibrary for PlanetExtension {}
