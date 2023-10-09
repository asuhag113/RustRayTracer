use std::ops;

#[derive(Clone, Copy)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}
impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }
    pub fn length_squared(&self) -> f32 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }
    pub fn length(&self) -> f32 {
        f32::sqrt(self.length_squared())
    }
    pub fn print(&self) {
        print!("{} {} {}", self.x, self.y, self.x)
    }
}
#[test]
fn test_new() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(a.x(), 1.0);
    assert_eq!(a.x(), 2.0);
    assert_eq!(a.x(), 3.0);
}
#[test]
fn test_length_squared() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = a.length_squared();
    assert_eq!(b, 14.0);
}
#[test]
fn test_length() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = a.length();
    assert_eq!(b, f32::sqrt(14.0));
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        return Vec3::new(-self.x(), -self.y(), -self.z());
    }
}
#[test]
fn test_neg() {
    let a = Vec3::new(1.0, 1.0, 1.0);
    let b = -a;
    assert_eq!(b.x(), -1.0);
    assert_eq!(b.y(), -1.0);
    assert_eq!(b.z(), -1.0);
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        return Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z());
    }
}
#[test]
fn test_add() {
    let a = Vec3::new(0.0, 1.0, 2.0);
    let b = Vec3::new(1.0, 1.0, 1.0);
    let c = a + b;
    assert_eq!(c.x(), 1.0);
    assert_eq!(c.y(), 2.0);
    assert_eq!(c.z(), 3.0);
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
#[test]
fn test_add_assign() {
    let mut a = Vec3::new(0.0, 1.0, 2.0);
    let b = Vec3::new(1.0, 1.0, 1.0);
    a += b;
    assert_eq!(a.x(), 1.0);
    assert_eq!(a.y(), 2.0);
    assert_eq!(a.z(), 3.0);
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        return Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z());
    }
}
#[test]
fn test_sub() {
    let a = Vec3::new(0.0, 1.0, 2.0);
    let b = Vec3::new(1.0, 1.0, 1.0);
    let c = a - b;
    assert_eq!(c.x(), -1.0);
    assert_eq!(c.y(), 0.0);
    assert_eq!(c.z(), 1.0);
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        return Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z());
    }
}
#[test]
fn test_vec_mul() {
    let a = Vec3::new(0.0, 1.0, 2.0);
    let b = Vec3::new(1.0, 1.0, 1.0);
    let c = a * b;
    assert_eq!(c.x(), 0.0);
    assert_eq!(c.y(), 1.0);
    assert_eq!(c.z(), 2.0);
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        return Vec3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs);
    }
}
#[test]
fn test_f32_rhs_mul() {
    let a = Vec3::new(0.0, 1.0, 2.0);
    let b = 2.0;
    let c = a * b;
    assert_eq!(c.x(), 0.0);
    assert_eq!(c.y(), 2.0);
    assert_eq!(c.z(), 4.0);
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        return Vec3::new(self * rhs.x(), self * rhs.y(), self * rhs.z());
    }
}

#[test]
fn test_f32_lhs_mul() {
    let a = Vec3::new(0.0, 1.0, 2.0);
    let b = 2.0;
    let c = b * a;
    assert_eq!(c.x(), 0.0);
    assert_eq!(c.y(), 2.0);
    assert_eq!(c.z(), 4.0);
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
#[test]
fn test_f32_mul_assign() {
    let mut a = Vec3::new(0.0, 1.0, 2.0);
    let b = 2.0;
    a *= b;
    assert_eq!(a.x(), 0.0);
    assert_eq!(a.y(), 2.0);
    assert_eq!(a.z(), 4.0);
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        return self * (1.0 / rhs);
    }
}
#[test]
fn test_f32_div() {
    let a = Vec3::new(4.0, 2.0, 0.0);
    let b = 2.0;
    let c = a / b;
    assert_eq!(c.x(), 2.0);
    assert_eq!(c.y(), 1.0);
    assert_eq!(c.z(), 0.0);
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self *= 1.0 / rhs;
    }
}
#[test]
fn test_f32_div_assign() {
    let mut a = Vec3::new(4.0, 2.0, 0.0);
    let b = 2.0;
    a /= b;
    assert_eq!(a.x(), 2.0);
    assert_eq!(a.y(), 1.0);
    assert_eq!(a.z(), 0.0);
}

trait Dot {
    fn dot(self, rhs: Vec3) -> f32;
}
impl Dot for Vec3 {
    fn dot(self, rhs: Vec3) -> f32 {
        return self.x * rhs.x() + self.y * rhs.y() + self.z * rhs.z();
    }
}
#[test]
fn test_dot() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(3.0, 2.0, 1.0);
    let c = a.dot(b);
    assert_eq!(c, 10.0)
}

trait Cross {
    fn cross(self, rhs: Vec3) -> Vec3;
}
impl Cross for Vec3 {
    fn cross(self, rhs: Vec3) -> Vec3 {
        return Vec3::new(
            self.y * rhs.z() - self.z * rhs.y(),
            self.z * rhs.x() - self.x * rhs.z(),
            self.x * rhs.y() - self.y * rhs.x()
        );
    }
}
#[test]
fn test_cross() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(3.0, 2.0, 1.0);
    let c = a.cross(b);
    assert_eq!(c.x(), -4.0);
    assert_eq!(c.y(), 8.0);
    assert_eq!(c.z(), -4.0)
}

trait UnitVec {
    fn unit_vec(self) -> Vec3;
}
impl UnitVec for Vec3 {
    fn unit_vec(self) -> Vec3 {
        return self / self.length();
    }
}
#[test]
fn test_unit_vec() {
    let a = Vec3::new(1.0, 0.0, 1.0);
    let b = a.unit_vec();
    assert_eq!(b.x(), 1.0 / f32::sqrt(2.0));
    assert_eq!(b.y(), 0.0);
    assert_eq!(b.z(), 1.0 / f32::sqrt(2.0));
}
