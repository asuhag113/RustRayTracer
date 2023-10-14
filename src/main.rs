use std::{
    io::{ Error, stdin, Write, self },
    path::Path,
    fs::{ File, create_dir_all },
    str::FromStr,
};

use raytracer::{ color::Color, ray::Ray, point3d::Point3D, vec3::UnitVec, camera::Camera };

/// returns a parsed user input that matches the type of the variable that is being assigned the input value
///
/// # Parameters
/// prompt - a custom prompt that is displayed in the command line to give context for the type of value desired
/// retry - a custom message displayed if the user's input value was not validated
/// validator - an optional function that further refines the type of input desired by the user
fn get_input<U: FromStr>(prompt: &str, retry: &str, validator: Option<fn(&str) -> bool>) -> U {
    loop {
        let mut input = String::new();

        println!("{}", prompt);
        stdin().read_line(&mut input).expect("Input should be read in from user");
        let custom_validator = validator.unwrap_or(|_| {
            return true;
        });
        let input = match input.trim().parse::<U>() {
            Ok(parsed_input) if custom_validator(&input) => parsed_input,
            _ => {
                println!("{}", retry);
                continue;
            }
        };
        return input;
    }
}

/// creates a file at the given filepath and writes the proper metadata for an image in the PPM format
fn write_to_file(
    filepath: &str,
    rgb_triplets: &Vec<[i32; 3]>,
    image_width: &usize,
    image_height: &usize
) -> Result<(), Error> {
    let mut file = File::create(filepath)?;
    file.write(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes()).expect(
        "Failed writing to file"
    );
    for i in 0..*image_width * image_height {
        let ir = rgb_triplets[i][0];
        let ig = rgb_triplets[i][1];
        let ib = rgb_triplets[i][2];
        file.write(format!("{} {} {}\n", ir, ig, ib).as_bytes()).expect("Failed writing to file");
    }
    Ok(())
}

/// calculates the rgb values to be written in the PPM file
fn render_rgb_triplets(
    rgb_triplets: &mut Vec<[i32; 3]>,
    image_width: &usize,
    image_height: &usize
) {
    let camera = Camera::new(
        Point3D::new(0.0, 0.0, 0.0), // +x is right, +y is up, -z is toward scene
        2.0 * ((*image_width as f32) / (*image_height as f32)), // use the calculated image height and width instead of the aspect ratio so the viewport proportions exactly match the image proportions
        2.0, // for now, we arbitarily set the viewport height to 2.0
        1.0
    );
    // horizontal delta vectors from pixel to pixel
    let pixel_delta_u = camera.viewport_u() / (*image_width as f32);
    // vertical delta vectors from pixel to pixel
    let pixel_delta_v = camera.viewport_v() / (*image_height as f32);

    // // Calculate the location of the upper left pixel
    let pixel00_loc = camera.viewport_upper_left() + 0.5 * (pixel_delta_u + pixel_delta_v);
    println!("Beginning render for {}x{}", image_width, image_height);
    let mut idx = 0;
    for j in 0..*image_height {
        print!("Scanlines remaining: {}\r", image_height - j);
        io::stdout().flush().unwrap();
        for i in 0..*image_width {
            let pixel_center =
                pixel00_loc + (i as f32) * pixel_delta_u + (j as f32) * pixel_delta_v;
            let ray_direction = pixel_center - camera.center();
            let ray = Ray::new(camera.center(), ray_direction);

            let pixel_color = ray_color(&ray);
            let rgb_triplet = pixel_color.as_i32();
            rgb_triplets[idx] = rgb_triplet;
            idx += 1;
        }
    }
    println!("\nFinished render");
}

fn ray_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction().unit_vec();
    // lerp between blue and white: (1−𝑎) * startValue + 𝑎 * endValue
    let a = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
}

fn main() -> Result<(), Error> {
    println!("~~RUST RAYTRACER~~");
    let output_dir: String = get_input(
        "Enter the filepath to the directory for the output image file",
        "Please enter a valid path",
        Some(|input| {
            let path = Path::new(input.trim());
            return !path.is_file();
        })
    );
    let image_width: usize = get_input(
        "Enter a width for the output image:",
        "Please enter a valid number",
        None
    );
    // for now, the ideal aspect ratio is 16:9
    // we try for a best-effort approximation by using an integer-based ratio of image width over image height
    let aspect_ratio: f32 = 16.0 / 9.0;
    // calculate the image height based on the provided width to ensure we match the aspect ratio, ensure that the height is at least 1
    let image_height = (if (image_width as f32) / aspect_ratio < 1.0 {
        1.0
    } else {
        (image_width as f32) / aspect_ratio
    }) as usize;

    let image_filename = String::from("image.ppm");

    // check if the path provided exists, if not, create the proper directories needed
    if !Path::new(&output_dir).exists() {
        create_dir_all(output_dir.clone())?;
    }

    // define the proper filepath based on trailing slashes
    let output_path = if output_dir.ends_with("/") {
        format!("{}{}", output_dir, image_filename)
    } else {
        format!("{}/{}", output_dir, image_filename)
    };

    // create a matrix to store our rgb triplets
    let mut rgb_triplets: Vec<[i32; 3]> = vec![[0; 3]; image_height * image_width];
    render_rgb_triplets(&mut rgb_triplets, &image_width, &image_height);
    write_to_file(&output_path, &rgb_triplets, &image_width, &image_height).expect(
        "Failed writing to file"
    );

    println!("Finished writing to {}", output_path);
    Ok(())
}
