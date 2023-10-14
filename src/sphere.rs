use crate::{ point3d::Point3D, hittable::{ Hittable, HitRecord }, vec3::Dot };

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
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        ray_tmin: f32,
        ray_tmax: f32,
        hit_record: &mut HitRecord
    ) -> bool {
        // formula for ray-sphere intersection
        // note, for now there is an intentional bug where the camera+scene cannot tell if the sphere is
        // in front of the camera (-z) or behind the camera (+z), so a sphere with z +1 and -1 will look the same
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = f32::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (-half_b + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.p = ray.at(hit_record.t);
        let outward_normal = (hit_record.p - self.center) / self.radius;
        hit_record.set_face_normal(ray, &outward_normal);

        return true;
    }
}
