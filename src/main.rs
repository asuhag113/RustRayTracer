use std::{ io::Error, rc::Rc };

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

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(
        Rc::new(Sphere::new(Point3D::new(0.0, -100.5, -1.0), 100.0, material_ground.clone()))
    );
    world.add(Rc::new(Sphere::new(Point3D::new(0.0, 0.0, -1.0), 0.5, material_center.clone())));
    world.add(Rc::new(Sphere::new(Point3D::new(-1.0, 0.0, -1.0), 0.5, material_left.clone())));
    world.add(Rc::new(Sphere::new(Point3D::new(-1.0, 0.0, -1.0), -0.4, material_left.clone())));
    world.add(Rc::new(Sphere::new(Point3D::new(1.0, 0.0, -1.0), 0.5, material_right.clone())));

    let camera = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        90.0,
        Point3D::new(-2.0, 2.0, 1.0),
        Point3D::new(0.0, 0.0, -1.0),
        // camera-relative "up" direction, allowing for rotation
        Vec3::new(0.0, 1.0, 0.0)
    );

    camera.render(&world);

    Ok(())
}
