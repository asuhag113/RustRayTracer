use rand::random;

use crate::{ ray::Ray, hittable::HitRecord, color::Color, vec3::{ Vec3, UnitVec, Dot } };

// This is intended to be implemented in any struct that describes a material and scatters rays
pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo: Color,
    pub fuzziness: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzziness: f32) -> Metal {
        Metal { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = Vec3::reflect(ray_in.direction().unit_vec(), hit_rec.normal);
        let scattered = Ray::new(
            hit_rec.p,
            reflected + self.fuzziness * Vec3::random_unit_vector()
        );
        let attenuation = self.albedo;
        return if scattered.direction().dot(hit_rec.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        };
    }
}

#[derive(Clone, Copy)]
pub struct Dielectric {
    pub refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Dielectric {
        Dielectric { refraction_index }
    }
    pub fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
        // use schlick's approximation for reflectance allowing for a material with reflectivity that varies with angle
        let r0 =
            ((1.0 - refraction_index) / (1.0 + refraction_index)) *
            ((1.0 - refraction_index) / (1.0 + refraction_index));
        return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit_rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.direction().unit_vec();
        let cos_theta = f32::min(-unit_direction.dot(hit_rec.normal), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

        // when the ray is in the material with the higher refractive index there is no real solution
        // to snell's law, so the material must reflect (total internal reflection)
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if
            cannot_refract ||
            Dielectric::reflectance(cos_theta, refraction_ratio) > random::<f32>()
        {
            Vec3::reflect(unit_direction, hit_rec.normal)
        } else {
            Vec3::refract(unit_direction, hit_rec.normal, refraction_ratio)
        };

        let scattered = Ray::new(hit_rec.p, direction);
        return Some((scattered, attenuation));
    }
}
