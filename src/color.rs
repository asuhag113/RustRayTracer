use crate::vec3::Vec3;

pub struct Color(Vec3);

impl Color {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Color(Vec3::new(x, y, z))
    }
    pub fn as_i32(&self) -> [i32; 3] {
        return [
            (255.999 * self.0.x()) as i32,
            (255.999 * self.0.y()) as i32,
            (255.999 * self.0.z()) as i32,
        ];
    }
}
