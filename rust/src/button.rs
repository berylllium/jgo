use godot::{classes::{AnimationPlayer, InputEvent, InputEventMouseButton}, global::MouseButton, prelude::*};

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct PushButton {
    #[export]
    toggle: bool,

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
    pub fn on_area_event(&mut self, _: Gd<Node>, event: Gd<InputEvent>, _: Vector3, _: Vector3, _: i64) {
        if let Ok(m) = event.try_cast::<InputEventMouseButton>() {
            if m.get_button_index() == MouseButton::LEFT && m.is_pressed() {
                if self.toggle {
                    if self.is_pressed { self.release(); } else { self.press(); }
                } else {
                    if !self.is_pressed { self.press(); }
                }
            }
        }
    }

    #[func]
    pub fn press(&mut self) {
        self.is_pressed = true;
        self.signals().button_released().emit();
        self.animation_player.as_mut().unwrap().play_ex().name("press").done();
    }

    #[func]
    pub fn release(&mut self) {
        self.is_pressed = false;
        self.signals().button_pressed().emit();
        self.animation_player.as_mut().unwrap().play_ex().name("release").done();
    }
}

#[godot_api]
impl INode3D for PushButton {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            toggle: false,
            animation_player: None,
            is_pressed: false,
            base,
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
