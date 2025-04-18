use std::f32::consts::PI;

use godot::prelude::*;

use crate::math::move_toward;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Jumpgate {
    #[export]
    inner_ring: Option<Gd<Node3D>>,
    #[export]
    outer_ring: Option<Gd<Node3D>>,

    pub ring_acceleration: f32,
    pub target_inner_velocity: f32,
    pub target_outer_velocity: f32,
    inner_velocity: f32,
    outer_velocity: f32,

    base: Base<Node3D>,
}

#[godot_api]
impl Jumpgate {
    fn handle_rotation(&mut self, delta: f32) {
        if self.inner_velocity != self.target_inner_velocity {
            self.inner_velocity = move_toward(
                self.inner_velocity,
                self.target_inner_velocity,
                self.ring_acceleration * delta,
            );
        }

        if self.outer_velocity != self.target_outer_velocity {
            self.outer_velocity = move_toward(
                self.outer_velocity,
                self.target_outer_velocity,
                self.ring_acceleration * delta,
            );
        }

        if self.inner_velocity > 0.0 {
            self.inner_ring.as_mut().unwrap().rotate_z(self.inner_velocity * delta);
        }

        if self.outer_velocity > 0.0 {
            self.outer_ring.as_mut().unwrap().rotate_x(self.outer_velocity * delta);
        }
    }
}

#[godot_api]
impl INode3D for Jumpgate {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            inner_ring: None,
            outer_ring: None,
            ring_acceleration: PI / 10.0,
            target_inner_velocity: 0.0,
            target_outer_velocity: 0.0,
            inner_velocity: 0.0,
            outer_velocity: 0.0,
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        let delta = delta as f32;

        self.handle_rotation(delta);
    }
}
