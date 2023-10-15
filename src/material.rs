use crate::{ ray::Ray, hittable::HitRecord, color::Color, vec3::{ Vec3, UnitVec } };

// This is intended to be implemented in any struct that describes a material and scatters rays
pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Color)>;
}

// diffuse
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    // we choose to always scatter and attenuate by R (reflectance)
    // alternatively, we can sometimes scatter (with probabilty 1 − R) with no attenuation
    // or scatter with some fixed probability p and have attenuation be albedo/p
    fn scatter(&self, _ray_in: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_dir = hit_rec.normal + Vec3::random_unit_vector();

        // catch the case where our random unit vector is opposite to the normal causing 0 scatter direction
        if scatter_dir.near_zero() {
            scatter_dir = hit_rec.normal;
        }

        let scattered = Ray::new(hit_rec.p, scatter_dir);
        let attenuation = self.albedo;
        return Some((scattered, attenuation));
    }
}

pub struct Metal {
    pub albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = Vec3::reflect(ray_in.direction().unit_vec(), hit_rec.normal);
        let scattered = Ray::new(hit_rec.p, reflected);
        let attenuation = self.albedo;
        return Some((scattered, attenuation));
    }
}
