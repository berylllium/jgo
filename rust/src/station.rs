mod jumpgate;

use godot::prelude::*;

use crate::game::Game;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Station {
    #[export]
    game: Option<Gd<Game>>,

    rotation: Vector3,

    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for Station {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            game: None,
            rotation: Vector3::ZERO,
            base,
        }
    }
}
