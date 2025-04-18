use godot::{
    classes::{CanvasItem, Sprite3D, SubViewport},
    prelude::*,
};

#[derive(GodotClass)]
#[class(tool, base=Node3D)]
pub struct Screen {
    #[export]
    ui: Option<Gd<PackedScene>>,
    #[export(range = (0.0, 10000.0, or_greater))]
    x: i32,
    #[export(range = (0.0, 10000.0, or_greater))]
    y: i32,
    #[export(range = (0.0, 10.0, or_greater))]
    width: f32,

    sprite: Option<Gd<Sprite3D>>,
    viewport: Option<Gd<SubViewport>>,
    cached_ui: Option<Gd<CanvasItem>>,

    is_on: bool,

    base: Base<Node3D>,
}

#[godot_api]
impl Screen {
    #[func]
    pub fn apply_size(&mut self) {
        if let (Some(sprite), Some(viewport)) = (self.sprite.as_mut(), self.viewport.as_mut()) {
            sprite.set_region_rect(Rect2 {
                position: Vector2::new(0.0, 0.0),
                size: Vector2::new(self.x as f32, self.y as f32),
            });
            sprite.set_pixel_size(self.width / self.x as f32);
            viewport.set_size(Vector2i::new(self.x, self.y));
        }
    }

    #[func]
    pub fn clear_ui(&mut self) {
        if let (Some(viewport), Some(ui)) = (self.viewport.as_mut(), self.cached_ui.as_mut()) {
            viewport.remove_child(ui as &_);
            ui.queue_free();
            self.cached_ui = None;
        }
    }

    #[func]
    pub fn change_ui(&mut self, ui: Gd<PackedScene>) {
        if let Some(mut viewport) = self.viewport.clone() {
            // Remove current ui if exists.
            self.clear_ui();

            // Allocate and assign new ui.
            let ui = ui.instantiate().map(|i| i.cast::<CanvasItem>()).unwrap();
            viewport.add_child(&ui);
            viewport.move_child(&ui, 0);
            self.cached_ui = Some(ui);
        }
    }

    #[func]
    pub fn turn_on(&mut self) {
        if let Some(ui) = self.cached_ui.as_mut() {
            ui.set_visible(true);
        }
    }

    #[func]
    pub fn turn_off(&mut self) {
        if let Some(ui) = self.cached_ui.as_mut() {
            ui.set_visible(false);
        }
    }

    #[func]
    pub fn on_off_toggle(&mut self) {
        if self.is_on {
            self.turn_off();
        } else {
            self.turn_on();
        }
    }
}

#[godot_api]
impl INode3D for Screen {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            ui: None,
            x: 100,
            y: 100,
            width: 1.0,
            sprite: None,
            viewport: None,
            cached_ui: None,
            is_on: true,
            base,
        }
    }

    fn ready(&mut self) {
        self.sprite = self.base().get_child(0).map(|n| n.cast());
        self.viewport = self.base().get_child(1).map(|n| n.cast());

        // Setup ui scene.
        if let Some(ui) = self.ui.clone() {
            self.change_ui(ui);
        }

        if self.is_on {
            self.turn_on();
        } else {
            self.turn_off();
        }

        self.apply_size();
    }

    fn set_property(&mut self, property: StringName, value: Variant) -> bool {
        if property == "ui".into() {
            self.ui = value.to();
            if let Some(ui) = self.ui.clone() {
                self.change_ui(ui);
            } else {
                self.clear_ui();
            }
            true
        } else if property == "x".into() {
            self.x = value.to();
            self.apply_size();
            true
        } else if property == "y".into() {
            self.y = value.to();
            self.apply_size();
            true
        } else if property == "width".into() {
            self.width = value.to();
            self.apply_size();
            true
        } else {
            false
        }
    }
}
