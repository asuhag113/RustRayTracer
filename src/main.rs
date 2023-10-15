use std::{ io::Error, rc::Rc };

use raytracer::{ point3d::Point3D, camera::Camera, hittable_list::HittableList, sphere::Sphere };

fn main() -> Result<(), Error> {
    println!("~~RUST RAYTRACER~~");

    // create a list of hittable objects in our scene
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3D::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3D::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new(16.0 / 9.0, 400, 100);

    camera.render(&world);

    Ok(())
}
