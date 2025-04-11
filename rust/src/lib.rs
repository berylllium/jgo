mod button;
mod game;
mod lever;
mod math;
mod player;
mod station;

use godot::prelude::*;

struct JGO;

#[gdextension]
unsafe impl ExtensionLibrary for JGO {}
