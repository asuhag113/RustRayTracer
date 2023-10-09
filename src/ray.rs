use crate::{ point3d::Point3D, vec3::Vec3 };

pub struct Ray {
    origin: Point3D,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }
    pub fn origin(&self) -> Point3D {
        return self.origin;
    }
    pub fn direction(&self) -> Vec3 {
        return self.direction;
    }
    pub fn at(&self, t: f32) -> Point3D {
        return self.origin + t * self.direction;
    }
}

#[test]
fn test_ray_new() {
    let o = Point3D::new(1.0, 1.0, 1.0);
    let d = Vec3::new(1.0, 0.0, 0.0);
    let r = Ray::new(o, d);
    assert_eq!(r.origin().x(), 1.0);
    assert_eq!(r.origin().y(), 1.0);
    assert_eq!(r.origin().z(), 1.0);
    assert_eq!(r.direction().x(), 1.0);
    assert_eq!(r.direction().y(), 0.0);
    assert_eq!(r.direction().z(), 0.0);
}

#[test]
fn test_ray_at() {
    let o = Point3D::new(1.0, 1.0, 1.0);
    let d = Vec3::new(1.0, 0.0, 0.0);
    let r = Ray::new(o, d);
    let at = r.at(1.0);
    assert_eq!(at.x(), 2.0);
    assert_eq!(at.y(), 1.0);
    assert_eq!(at.z(), 1.0);
}
