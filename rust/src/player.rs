use std::f32::consts::PI;

use godot::classes::input::MouseMode;
use godot::classes::{
    Camera3D, CharacterBody3D, ICharacterBody3D, Input, InputEvent, InputEventMouseButton, InputEventMouseMotion,
};
use godot::prelude::*;

use crate::math::{clamp, move_toward};

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct Player {
    #[export]
    #[var(set = set_fov)]
    fov: f32,
    base_fov: f32,

    speed: f32,

    mouse_sensitivity: f32,

    in_precision_mode: bool,
    zoom_step: f32,

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

    fn handle_precision_mode(&mut self, event: Gd<InputEvent>) {
        // Enter exit precision mode.
        if event.is_action_pressed("precision_mode") && !self.in_precision_mode {
            self.enter_precision_mode();
        } else if event.is_action_released("precision_mode") && self.in_precision_mode {
            self.exit_precision_mode();
        }

        if !self.in_precision_mode {
            return;
        }

        // Zoom.
        if event.is_action_pressed("precision_mode_zoom_in") {
            self.zoom_in();
        } else if event.is_action_pressed("precision_mode_zoom_out") {
            self.zoom_out();
        }
    }
}

#[godot_api]
impl Player {
    const ZOOM_MIN: f32 = 5.0;

    #[func]
    fn enter_precision_mode(&mut self) {
        self.in_precision_mode = true;
        Input::singleton().set_mouse_mode(MouseMode::CONFINED);
    }

    #[func]
    fn exit_precision_mode(&mut self) {
        self.in_precision_mode = false;
        Input::singleton().set_mouse_mode(MouseMode::CAPTURED);

        // Reset zoom.
        self.set_fov(self.base_fov);
    }

    #[func]
    fn zoom_in(&mut self) {
        self.set_fov(self.fov - self.zoom_step);
    }

    #[func]
    fn zoom_out(&mut self) {
        self.set_fov(self.fov + self.zoom_step);
    }

    #[func]
    pub fn set_fov(&mut self, fov: f32) {
        self.fov = clamp(fov, Self::ZOOM_MIN, self.base_fov);
        if let Some(camera) = self.camera.as_mut() {
            camera.set_fov(self.fov);
        }
    }
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            fov: 75.0,
            base_fov: 75.0,
            speed: 250.0,
            mouse_sensitivity: 0.001,
            in_precision_mode: false,
            zoom_step: 5.0,
            camera: None,
            base,
        }
    }

    fn ready(&mut self) {
        Input::singleton().set_mouse_mode(MouseMode::CAPTURED);

        self.camera = self.base().get_child(0).map(|n| n.cast::<Camera3D>());
        self.set_fov(self.fov);
        self.base_fov = self.fov;
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        self.handle_precision_mode(event.clone());
        self.handle_camera(event);
    }

    fn process(&mut self, delta: f64) {
        self.handle_movement(delta);
    }
}
