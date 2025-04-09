pub fn move_toward(from: f32, to: f32, delta: f32) -> f32 {
    let r = from + delta;

    if r > to { to } else { r }
}

pub fn clamp(v: f32, min: f32, max: f32) -> f32 {
    let t = if v < min { min } else { v };
    if t > max { max } else { t }
}
