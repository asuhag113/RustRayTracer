use crate::point3d::Point3D;

pub struct Camera {
    center: Point3D,
    viewport_width: f32,
    viewport_height: f32,
    focal_length: f32,
}

impl Camera {
    pub fn new(
        center: Point3D,
        viewport_width: f32,
        viewport_height: f32,
        focal_length: f32
    ) -> Camera {
        Camera {
            center,
            viewport_width,
            viewport_height,
            focal_length,
        }
    }
    pub fn center(&self) -> Point3D {
        return self.center;
    }
    pub fn viewport_width(&self) -> f32 {
        return self.viewport_width;
    }
    pub fn viewport_height(&self) -> f32 {
        return self.viewport_height;
    }
    pub fn focal_length(&self) -> f32 {
        return self.focal_length;
    }
    // horizontal vector on viewport edge
    pub fn viewport_u(&self) -> Point3D {
        return Point3D::new(self.viewport_width, 0.0, 0.0);
    }
    // vertical vector on viewport edge
    pub fn viewport_v(&self) -> Point3D {
        return Point3D::new(0.0, -self.viewport_height, 0.0);
    }
    pub fn viewport_upper_left(&self) -> Point3D {
        return self.center -
            Point3D::new(0.0, 0.0, self.focal_length) -
            self.viewport_u() / 2.0 -
            self.viewport_v() / 2.0;
    }
}

#[test]
fn test_new() {
    let c = Camera::new(Point3D::new(0.0, 0.0, 0.0), 100.0, 100.0, 1.0);
    assert_eq!(c.center().x(), 0.0);
    assert_eq!(c.center().y(), 0.0);
    assert_eq!(c.center().z(), 0.0);
    assert_eq!(c.viewport_width(), 100.0);
    assert_eq!(c.viewport_height(), 100.0);
    assert_eq!(c.focal_length(), 1.0);
    assert_eq!(c.viewport_u().x(), 100.0);
    assert_eq!(c.viewport_u().y(), 0.0);
    assert_eq!(c.viewport_u().z(), 0.0);
    assert_eq!(c.viewport_v().x(), 0.0);
    assert_eq!(c.viewport_v().y(), -100.0);
    assert_eq!(c.viewport_v().z(), 0.0);
    assert_eq!(c.viewport_upper_left().x(), 0.0 - 0.0 - 100.0 / 2.0 - 0.0 / 2.0);
    assert_eq!(c.viewport_upper_left().y(), 0.0 - 0.0 - 0.0 / 2.0 - -100.0 / 2.0);
    assert_eq!(c.viewport_upper_left().z(), 0.0 - 1.0 - 0.0 / 2.0 - 0.0 / 2.0);
}
