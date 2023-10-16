use std::{ io::{ self, Write }, fs::File };

use rand::random;

use crate::{
    point3d::Point3D,
    hittable_list::HittableList,
    ray::Ray,
    vec3::{ Vec3, UnitVec, Cross },
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
    samples_per_pixel: i32,
    max_depth: i32,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    defocus_angle: f32,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        image_width: i32,
        samples_per_pixel: i32,
        max_depth: i32,
        vfov: f32,
        look_from: Point3D,
        look_at: Point3D,
        vup: Vec3,
        defocus_angle: f32,
        focus_distance: f32
    ) -> Camera {
        // Calculate the image height based on the provided width to ensure we match the aspect ratio, ensure that the height is at least 1
        let image_height = if (image_width as f32) / aspect_ratio < 1.0 {
            1
        } else {
            ((image_width as f32) / aspect_ratio) as i32
        };

        let center = look_from;

        // Determine viewport dimensions
        let theta = f32::to_radians(vfov);
        let h = f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h * focus_distance;
        let viewport_width =
            viewport_height * (((image_width as f32) / (image_height as f32)) as f32);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame
        let w = (look_from - look_at).unit_vec();
        let u = vup.cross(w).unit_vec();
        let v = w.cross(u);

        // Calculate the viewport edge vectors
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculate the delta vectors between pixels
        let pixel_delta_u = viewport_u / (image_width as f32);
        let pixel_delta_v = viewport_v / (image_height as f32);

        // Calculate the location of the upper left pixel
        let viewport_upper_left = center - focus_distance * w - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camrera defocus disk basis vectors
        let defocus_radius = focus_distance * f32::tan(f32::to_radians(defocus_angle / 2.0));
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        return Camera {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            max_depth,
            defocus_disk_u,
            defocus_disk_v,
            defocus_angle,
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
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += self.ray_color(&ray, self.max_depth, world);
                }
                let scale = 1.0 / (self.samples_per_pixel as f32);
                // images are generally stored in gamma space, so here we convert our linear values into the gamma space
                // for more accurate color intensity when viewing the image in image editors
                let r = Color::linear_to_gamma(pixel_color.x() * scale);
                let g = Color::linear_to_gamma(pixel_color.y() * scale);
                let b = Color::linear_to_gamma(pixel_color.z() * scale);

                let intensity = Interval::new(0.0, 0.999);

                file.write(
                    format!(
                        "{} {} {}\n",
                        (256.0 * intensity.clamp(r)) as i32,
                        (256.0 * intensity.clamp(g)) as i32,
                        (256.0 * intensity.clamp(b)) as i32
                    ).as_bytes()
                ).expect("Failed writing to file");
            }
        }
        println!("\nFinished render");
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Get a randomly-sampled camera ray for the pixel at i,j originating
        // from the camera defocus disk
        let pixel_center =
            self.pixel00_loc + (i as f32) * self.pixel_delta_u + (j as f32) * self.pixel_delta_v;
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        return Ray::new(ray_origin, ray_direction);
    }

    // returns a random point in the camera defocus disk
    fn defocus_disk_sample(&self) -> Point3D {
        let p = Vec3::random_in_unit_disk();
        return self.center + p.x() * self.defocus_disk_u + p.y() * self.defocus_disk_v;
    }

    // returns a random point in the square surrounding a pixel at the origin
    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random::<f32>();
        let py = -0.5 + random::<f32>();
        return px * self.pixel_delta_u + py * self.pixel_delta_v;
    }

    fn ray_color(&self, ray: &Ray, depth: i32, world: &HittableList) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        // Ignore hits that are very close to the calculated intersection point to prevent "shadow acne" from floating point rounding errors
        if let Some(hit) = world.hit(ray, &Interval::new(0.001, f32::INFINITY)) {
            if let Some((scattered, attenuation)) = hit.material.scatter(ray, &hit) {
                return attenuation * self.ray_color(&scattered, depth - 1, world);
            }
            return Color::new(0.0, 0.0, 0.0);
        }
        let unit_direction = ray.direction().unit_vec();
        // lerp between blue and white: (1−𝑎) * startValue + 𝑎 * endValue
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
    }
}
