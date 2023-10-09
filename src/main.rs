use std::{ io::{ Error, stdin, Write }, path::Path, fs::{ File, create_dir_all }, str::FromStr };

use raytracer::color::Color;

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
    println!("Beginning render for {}x{}", image_width, image_height);
    let mut idx = 0;
    for j in 0..*image_width {
        for i in 0..*image_height {
            let pixel_color = Color::new(
                (i as f32) / ((image_width - 1) as f32),
                (j as f32) / ((image_height - 1) as f32),
                0.0
            );
            let rgb_triplet = pixel_color.as_i32();
            rgb_triplets[idx] = rgb_triplet;
            idx += 1;
        }
    }
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
    let image_height: usize = get_input(
        "Enter a height for the output image:",
        "Please enter a valid number",
        None
    );
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
