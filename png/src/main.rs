use image::GenericImageView; // Required for methods like .dimensions() and .pixels()

fn main() -> Result<(), image::ImageError> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <image_file>", args[0]);
        std::process::exit(1);
    }
    let img_path = &args[1];

    // Open the image file
    let img = image::open(img_path)?;

    // Get image dimensions
    let (width, height) = img.dimensions();
    println!("Image dimensions: {}x{}", width, height);

    // Iterate over pixels (example: printing RGB values of the first pixel)
    if let Some(pixel) = img.pixels().next() {
        let (x, y, rgba) = pixel;
        println!("First pixel at ({}, {}): R={}, G={}, B={}, A={}", x, y, rgba[0], rgba[1], rgba[2], rgba[3]);
    }

    // You can also convert the image to a specific format if needed
    // For example, converting to RGBA8:
    let rgba_image = img.to_rgba8();

    // Access pixel data from the converted image
    // For example, getting the pixel at (0, 0)
    let first_pixel_rgba8 = rgba_image.get_pixel(0, 0);
    println!("First pixel (RGBA8): R={}, G={}, B={}, A={}", first_pixel_rgba8[0], first_pixel_rgba8[1], first_pixel_rgba8[2], first_pixel_rgba8[3]);

    Ok(())
}
