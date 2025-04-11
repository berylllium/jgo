use godot::{classes::{InputEvent, InputEventMouseButton, InputEventMouseMotion}, global::{deg_to_rad, MouseButton}, prelude::*};

use crate::math::clamp;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Lever {
    #[export]
    lever: Option<Gd<Node3D>>,
    /// Max rotation in both directions.
    max_rot: f32,
    rot_sensitivity: f32,
    is_held: bool,
    position: f32,
    base: Base<Node3D>,
}

impl Lever {
    fn update_rotation(&mut self) {
        self.lever.as_mut().unwrap().set_rotation(Vector3::new(self.max_rot * self.position, 0.0, 0.0));
    }
}

#[godot_api]
impl Lever {
    #[signal]
    fn lever_up();

    #[signal]
    fn lever_down();

    #[func]
    pub fn on_area_event(&mut self, _: Gd<Node>, event: Gd<InputEvent>, _: Vector3, _: Vector3, _: i64) {
        if let Ok(m) = event.try_cast::<InputEventMouseButton>() {
            if m.get_button_index() == MouseButton::LEFT && m.is_pressed() && !self.is_held {
                self.is_held = true;
            }
        }
    }
}

#[godot_api]
impl INode3D for Lever {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            lever: None,
            max_rot: deg_to_rad(59.0) as f32,
            rot_sensitivity: 0.005,
            is_held: false,
            position: 0.0,
            base,
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if !self.is_held { return; }

        if let Ok(m) = event.clone().try_cast::<InputEventMouseMotion>() {
            let newpos = clamp(self.position - self.rot_sensitivity * m.get_screen_relative().y, -1.0, 1.0);

            if self.position < 0.9 && newpos >= 0.9 {
                self.signals().lever_up().emit();
                godot_print!("Lever up.");
            } else if self.position > -0.9 && newpos <= -0.9 {
                self.signals().lever_down().emit();
                godot_print!("Lever down.");
            }

            self.position = newpos;
            self.update_rotation();
        } else if let Ok(m) = event.try_cast::<InputEventMouseButton>() {
            if m.get_button_index() == MouseButton::LEFT && !m.is_pressed() && self.is_held {
                self.is_held = false;
            }
        }
    }
}
