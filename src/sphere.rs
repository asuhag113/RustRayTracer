use crate::{
    point3d::Point3D,
    hittable::{ Hittable, HitRecord },
    vec3::Dot,
    interval::Interval,
    ray::Ray,
};

pub struct Sphere {
    center: Point3D,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3D, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        // formula for ray-sphere intersection
        // note, for now there is an intentional bug where the camera+scene cannot tell if the sphere is
        // in front of the camera (-z) or behind the camera (+z), so a sphere with z +1 and -1 will look the same
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = f32::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }
        let p = ray.at(root);
        // our normals always point against the ray, so we must store which side of the surface the ray is on
        // an alternative to this would be to determine the side of the surface during coloring
        let outward_normal = (p - self.center) / self.radius;
        let front_face = ray.direction().dot(outward_normal) < 0.0;
        return Some(
            HitRecord::new(
                p,
                if front_face {
                    outward_normal
                } else {
                    -outward_normal
                },
                root,
                front_face
            )
        );
    }
}
