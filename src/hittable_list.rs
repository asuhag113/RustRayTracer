use std::rc::Rc;

use crate::{ hittable::{ Hittable, HitRecord }, ray::Ray };

pub struct HittableList {
    // Rc is similar to shared_ptr in c++
    // shared pointers allow multiple geometries to share ea common instance (i.e. many spheres with same material)
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        return HittableList { objects: Vec::new() };
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object)
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        let mut closest_so_far = ray_tmax;
        let mut hit_record = None;
        for object in &self.objects {
            if let Some(hit) = object.hit(ray, ray_tmin, closest_so_far) {
                closest_so_far = hit.t;
                hit_record = Some(hit);
            }
        }

        return hit_record;
    }
}
