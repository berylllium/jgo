use godot::{
    classes::{AnimationPlayer, CollisionObject3D, InputEvent, InputEventMouseButton},
    global::MouseButton,
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct PushButton {
    #[export]
    toggle: bool,

    #[export]
    hitbox: Option<Gd<CollisionObject3D>>,

    #[export]
    animation_player: Option<Gd<AnimationPlayer>>,

    is_pressed: bool,

    base: Base<Node3D>,
}

#[godot_api]
impl PushButton {
    #[signal]
    fn button_pressed();

    #[signal]
    fn button_released();

    #[func]
    pub fn on_hitbox_event(&mut self, _: Gd<Node>, event: Gd<InputEvent>, _: Vector3, _: Vector3, _: i64) {
        if let Ok(m) = event.try_cast::<InputEventMouseButton>() {
            if m.get_button_index() == MouseButton::LEFT && m.is_pressed() {
                if self.toggle {
                    if self.is_pressed {
                        self.release();
                    } else {
                        self.press();
                    }
                } else {
                    if !self.is_pressed {
                        self.press();
                    }
                }
            }
        }
    }

    #[func]
    pub fn press(&mut self) {
        self.is_pressed = true;
        self.signals().button_released().emit();
        if let Some(animation_player) = self.animation_player.as_mut() {
            animation_player.play_ex().name("press").done();
        }
    }

    #[func]
    pub fn release(&mut self) {
        self.is_pressed = false;
        self.signals().button_pressed().emit();
        if let Some(animation_player) = self.animation_player.as_mut() {
            animation_player.play_ex().name("release").done();
        }
    }
}

#[godot_api]
impl INode3D for PushButton {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            toggle: false,
            hitbox: None,
            animation_player: None,
            is_pressed: false,
            base,
        }
    }

    fn ready(&mut self) {
        let callable = Callable::from_object_method(&self.to_gd(), "on_hitbox_event");

        if let Some(hitbox) = self.hitbox.as_mut() {
            hitbox.connect("input_event", &callable);
        }
    }

    // Check global input in case toggle is set to false and user moves mouse off of button before
    // releasing.
    fn input(&mut self, event: Gd<InputEvent>) {
        if let Ok(m) = event.try_cast::<InputEventMouseButton>() {
            if m.get_button_index() == MouseButton::LEFT && !m.is_pressed() && self.is_pressed && !self.toggle {
                self.release();
            }
        }
    }
}
