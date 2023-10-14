use crate::{ ray::Ray, point3d::Point3D, vec3::{ Vec3, Dot } };

pub struct HitRecord {
    pub p: Point3D,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    // our normals always point against the ray, so we must store which side of the surface the ray is on
    // an alternative to this would be to determine the side of the surface during coloring
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        // note, outward_normmal is assumed to have unit length
        self.front_face = ray.direction().dot(*outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }
}

// this trait is intended to be implemented for any "object" that a ray might hit
pub trait Hittable {
    // a ray only "counts" if it is within tmin and tmax
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32, hit_record: &mut HitRecord) -> bool;
}
