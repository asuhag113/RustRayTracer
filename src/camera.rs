use std::{ io::{ self, Write }, fs::File };

use crate::{
    point3d::Point3D,
    hittable_list::HittableList,
    ray::Ray,
    vec3::{ Vec3, UnitVec },
    hittable::Hittable,
    interval::Interval,
    color::Color,
};

pub struct Camera {
    image_width: i32,
    image_height: i32,
    center: Point3D,
    pixel00_loc: Point3D,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: i32) -> Camera {
        // Calculate the image height based on the provided width to ensure we match the aspect ratio, ensure that the height is at least 1
        let image_height = if (image_width as f32) / aspect_ratio < 1.0 {
            1
        } else {
            ((image_width as f32) / aspect_ratio) as i32
        };

        let center = Point3D::new(0.0, 0.0, 0.0);

        // Determine viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width =
            viewport_height * (((image_width as f32) / (image_height as f32)) as f32);

        // Calculate the viewport edge vectors
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the delta vectors between pixels
        let pixel_delta_u = viewport_u / (image_width as f32);
        let pixel_delta_v = viewport_v / (image_height as f32);

        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        return Camera {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        };
    }

    pub fn render(&self, world: &HittableList) {
        println!("Beginning render for {}x{}", self.image_width, self.image_height);
        let mut file = File::create("./output/image.ppm").expect("Failed creating file");
        file.write(
            format!("P3\n{} {}\n255\n", self.image_width, self.image_height).as_bytes()
        ).expect("Failed writing to file");
        for j in 0..self.image_height {
            print!("Scanlines remaining: {}\r", self.image_height - j);
            io::stdout().flush().unwrap();
            for i in 0..self.image_width {
                let pixel_center =
                    self.pixel00_loc +
                    (i as f32) * self.pixel_delta_u +
                    (j as f32) * self.pixel_delta_v;
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);

                let pixel_color = self.ray_color(&ray, world);
                let rgb_triplet = pixel_color.as_i32();
                file.write(
                    format!("{} {} {}\n", rgb_triplet[0], rgb_triplet[1], rgb_triplet[2]).as_bytes()
                ).expect("Failed writing to file");
            }
        }
        println!("\nFinished render");
    }

    fn ray_color(&self, ray: &Ray, world: &HittableList) -> Color {
        if let Some(hit) = world.hit(ray, &Interval::new(0.0, f32::INFINITY)) {
            return 0.5 * Color::new(1.0 + hit.normal.x(), 1.0 + hit.normal.y(), 1.0 + hit.normal.z());
        }
        let unit_direction = ray.direction().unit_vec();
        // lerp between blue and white: (1−𝑎) * startValue + 𝑎 * endValue
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
    }
}
