use std::ops;
use rand::random;

use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Color(Vec3);

impl Color {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Color(Vec3::new(x, y, z))
    }
    pub fn random() -> Color {
        return Color(Vec3::new(random::<f32>(), random::<f32>(), random::<f32>()));
    }
    pub fn random_in_range(min: f32, max: f32) -> Color {
        return Color(Vec3::random_in_range(min, max));
    }
    pub fn as_i32(&self) -> [i32; 3] {
        return [
            (255.999 * self.x()) as i32,
            (255.999 * self.y()) as i32,
            (255.999 * self.z()) as i32,
        ];
    }
    pub fn linear_to_gamma(lc: f32) -> f32 {
        return f32::sqrt(lc);
    }
}

// Deref defines how this wrapper type should behave when accessing the underlying value
// it alleviates us from having to always reference Vec3 traits and properties with Color.0
impl ops::Deref for Color {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        return Color::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z());
    }
}
#[test]
fn test_color_mul() {
    let a = Color::new(0.0, 1.0, 2.0);
    let b = Color::new(1.0, 1.0, 1.0);
    let c = a * b;
    assert_eq!(c.x(), 0.0);
    assert_eq!(c.y(), 1.0);
    assert_eq!(c.z(), 2.0);
}

impl ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Color(self.0 * rhs)
    }
}
#[test]
fn test_f32_rhs_mul() {
    let a = Color::new(0.0, 1.0, 2.0);
    let b = 2.0;
    let c = a * b;
    assert_eq!(c.x(), 0.0);
    assert_eq!(c.y(), 2.0);
    assert_eq!(c.z(), 4.0);
}

impl ops::Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        return Color::new(self * rhs.x(), self * rhs.y(), self * rhs.z());
    }
}
#[test]
fn test_f32_lhs_mul() {
    let a = Color::new(0.0, 1.0, 2.0);
    let b = 2.0;
    let c = b * a;
    assert_eq!(c.x(), 0.0);
    assert_eq!(c.y(), 2.0);
    assert_eq!(c.z(), 4.0);
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        return Color::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z());
    }
}
#[test]
fn test_add() {
    let a = Color::new(0.0, 1.0, 2.0);
    let b = Color::new(1.0, 1.0, 1.0);
    let c = a + b;
    assert_eq!(c.x(), 1.0);
    assert_eq!(c.y(), 2.0);
    assert_eq!(c.z(), 3.0);
}

impl ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.0 += rhs.0;
    }
}
#[test]
fn test_add_assign() {
    let mut a = Color::new(0.0, 1.0, 2.0);
    let b = Color::new(1.0, 1.0, 1.0);
    a += b;
    assert_eq!(a.x(), 1.0);
    assert_eq!(a.y(), 2.0);
    assert_eq!(a.z(), 3.0);
}
