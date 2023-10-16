use std::{ io::Error, rc::Rc };

use rand::Rng;
use raytracer::{
    point3d::Point3D,
    camera::Camera,
    hittable_list::HittableList,
    sphere::Sphere,
    material::{ Lambertian, Metal, Dielectric },
    color::Color,
    vec3::Vec3,
};

fn main() -> Result<(), Error> {
    println!("~~RUST RAYTRACER~~");

    // create a list of hittable objects in our scene
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(
        Rc::new(Sphere::new(Point3D::new(0.0, -1000.0, 0.0), 1000.0, ground_material.clone()))
    );

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let center = Point3D::new(
                (a as f32) + 0.9 * rng.gen::<f32>(),
                0.2,
                (b as f32) + 0.9 * rng.gen::<f32>()
            );

            if (center - Point3D::new(4.0, 0.2, 0.0)).length() > 0.9 {
                match choose_mat {
                    v if v < 0.8 => {
                        let albedo = Color::random() * Color::random();
                        let material = Rc::new(Lambertian::new(albedo));
                        world.add(Rc::new(Sphere::new(center, 0.2, material)));
                    }
                    v if v < 0.95 => {
                        let albedo = Color::random_in_range(0.5, 1.0);
                        let fuzziness = rng.gen_range(0.0..0.5);
                        let material = Rc::new(Metal::new(albedo, fuzziness));
                        world.add(Rc::new(Sphere::new(center, 0.2, material)));
                    }
                    _ => {
                        let material = Rc::new(Dielectric::new(1.5));
                        world.add(Rc::new(Sphere::new(center, 0.2, material)));
                    }
                }
            }
        }
    }

    let material_1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(Point3D::new(0.0, 1.0, 0.0), 1.0, material_1)));

    let material_2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(Point3D::new(-4.0, 1.0, 0.0), 1.0, material_2)));

    let material_3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(Point3D::new(4.0, 1.0, 0.0), 1.0, material_3)));

    let camera = Camera::new(
        16.0 / 9.0,
        1200,
        500,
        50,
        20.0,
        Point3D::new(13.0, 2.0, 3.0),
        Point3D::new(0.0, 0.0, 0.0),
        // camera-relative "up" direction, allowing for rotation
        Vec3::new(0.0, 1.0, 0.0),
        0.6,
        10.0
    );

    camera.render(&world);

    Ok(())
}
