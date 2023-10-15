use std::{ io::Error, rc::Rc };

use raytracer::{
    point3d::Point3D,
    camera::Camera,
    hittable_list::HittableList,
    sphere::Sphere,
    material::{ Lambertian, Metal },
    color::Color,
};

fn main() -> Result<(), Error> {
    println!("~~RUST RAYTRACER~~");

    // create a list of hittable objects in our scene
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    world.add(Rc::new(Sphere::new(Point3D::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Rc::new(Sphere::new(Point3D::new(0.0, 0.0, -1.0), 0.5, material_center)));
    world.add(Rc::new(Sphere::new(Point3D::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Rc::new(Sphere::new(Point3D::new(1.0, 0.0, -1.0), 0.5, material_right)));

    let camera = Camera::new(16.0 / 9.0, 400, 100, 50);

    camera.render(&world);

    Ok(())
}
