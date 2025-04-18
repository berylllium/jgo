use godot::{classes::ShaderMaterial, prelude::*};

#[derive(GodotClass)]
#[class(tool, base=Node3D)]
pub struct Game {
    #[export]
    skybox_material: Option<Gd<ShaderMaterial>>,

    system_star: Star,
    stars: Vec<Star>,

    base: Base<Node3D>,
}

#[godot_api]
impl Game {
    pub fn set_star_rotation(&mut self, rotation: Vector3) {
        let star_dirs = self
            .stars
            .iter()
            .map(|s| {
                s.space_location
                    .position
                    .rotated(Vector3::UP, rotation.y)
                    .rotated(Vector3::RIGHT, rotation.x)
                    .rotated(Vector3::BACK, rotation.z)
                    .normalized_or_zero()
            })
            .collect::<Vec<_>>();

        // TODO: Perhaps optimize?
        self.skybox_material
            .as_mut()
            .unwrap()
            .set_shader_parameter("stars", &PackedVector3Array::from(star_dirs.as_slice()).to_variant());
        self.skybox_material
            .as_mut()
            .unwrap()
            .set_shader_parameter("star_count", &(star_dirs.len() as u64).to_variant());
    }
}

#[godot_api]
impl INode3D for Game {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            skybox_material: None,
            system_star: Star {
                diameter: 1400000,
                space_location: SpaceLocation {
                    name: "Googer".into(),
                    position: Vector3::new(0.0, 0.0, 0.0),
                },
            },
            stars: vec![
                Star {
                    diameter: 1200000,
                    space_location: SpaceLocation {
                        name: "Goober".into(),
                        position: Vector3::new(4.0, 10.0, 3.0),
                    },
                },
                Star {
                    diameter: 1200000,
                    space_location: SpaceLocation {
                        name: "Gorgar".into(),
                        position: Vector3::new(5.0, 2.0, 0.0),
                    },
                },
                Star {
                    diameter: 1200000,
                    space_location: SpaceLocation {
                        name: "Stroober".into(),
                        position: Vector3::new(4.0, 2.0, 10.0),
                    },
                },
            ],
            base,
        }
    }

    fn ready(&mut self) {
        self.set_star_rotation(Vector3::ZERO);
    }
}

#[derive(Clone)]
pub struct Star {
    pub diameter: u64,
    pub space_location: SpaceLocation,
}

#[derive(Clone)]
pub struct SpaceLocation {
    pub name: String,
    pub position: Vector3,
}
