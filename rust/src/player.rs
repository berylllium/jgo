use std::f32::consts::PI;

use godot::classes::input::MouseMode;
use godot::prelude::*;
use godot::classes::{CharacterBody3D, ICharacterBody3D, InputEvent, InputEventMouseMotion};

use crate::math::{clamp, move_toward};

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct Player {
    speed: f32,

    mouse_sensitivity: f32,

    in_precision_mode: bool,

    #[export]
    camera: Option<Gd<Camera3D>>,

    base: Base<CharacterBody3D>,
}

impl Player {
    fn handle_movement(&mut self, delta: f64) {
        let mut velocity = self.base().get_velocity();

        if !self.base().is_on_floor() {
            velocity += self.base().get_gravity();
        }

        let input_dir =
            Input::singleton().get_vector("player_left", "player_right", "player_forward", "player_backward");
        let direction =
            (self.base().get_transform().basis * Vector3::new(input_dir.x, 0.0, input_dir.y)).normalized_or_zero();

        if direction != Vector3::ZERO {
            velocity.x = direction.x * self.speed * delta as f32;
            velocity.z = direction.z * self.speed * delta as f32;
        } else {
            let p_velocity = self.base().get_velocity();
            velocity.x = move_toward(p_velocity.x, 0.0, self.speed * delta as f32);
            velocity.z = move_toward(p_velocity.z, 0.0, self.speed * delta as f32);
        }

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
    }

    fn handle_camera(&mut self, event: Gd<InputEvent>) {
        if !self.in_precision_mode {
            if let (Ok(m), Some(mut camera)) = (event.try_cast::<InputEventMouseMotion>(), self.camera.clone()) {
                let screen_relative = -1.0 * self.mouse_sensitivity * m.get_screen_relative();

                self.base_mut().rotate_y(screen_relative.x);
                
                let camera_x_rotation = camera.get_rotation().x + screen_relative.y;
                camera.set_rotation(Vector3::new(clamp(camera_x_rotation, -0.5 * PI, 0.5 * PI), 0.0, 0.0));
            }
        }
    }

    fn handle_input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("precision_mode") && !self.in_precision_mode {
            self.enter_precision_mode();
        } else if event.is_action_released("precision_mode") && self.in_precision_mode {
            self.exit_precision_mode();
        }
    }

    fn enter_precision_mode(&mut self) {
        self.in_precision_mode = true;
        Input::singleton().set_mouse_mode(MouseMode::CONFINED);
    }

    fn exit_precision_mode(&mut self) {
        self.in_precision_mode = false;
        Input::singleton().set_mouse_mode(MouseMode::CAPTURED);
    }
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            speed: 250.0,
            mouse_sensitivity: 0.001,
            in_precision_mode: false,
            camera: None,
            base,
        }
    }

    fn ready(&mut self) {
        Input::singleton().set_mouse_mode(MouseMode::CAPTURED);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        self.handle_input(event.clone());
        self.handle_camera(event);
    }

    fn physics_process(&mut self, delta: f64) {
        self.handle_movement(delta);
    }
}
