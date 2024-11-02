use image::{DynamicImage, GenericImageView, RgbaImage, Rgba, imageops::FilterType};

// Function to load an image from a file path
pub fn load_image(path: &str) -> DynamicImage {
    image::open(path).expect("Failed to open image")
}

// Function to resize the default image to match the secret image dimensions
pub fn resize_image(image: &DynamicImage, width: u32, height: u32) -> DynamicImage {
    image.resize_exact(width, height, FilterType::Lanczos3)
}

// Apply Floyd-Steinberg dithering algorithm to the image
pub fn dither_image(image: &DynamicImage) -> RgbaImage {
    let (width, height) = image.dimensions();
    let mut dithered_image = image.to_rgba8();

    for y in 0..height {
        for x in 0..width {
            let old_pixel = dithered_image.get_pixel(x, y);
            let new_pixel = Rgba([
                if old_pixel[0] > 127 { 255 } else { 0 }, // Binary thresholding
                if old_pixel[1] > 127 { 255 } else { 0 },
                if old_pixel[2] > 127 { 255 } else { 0 },
                255, // Set alpha to fully opaque
            ]);

            let quant_error = [
                old_pixel[0] as f32 - new_pixel[0] as f32,
                old_pixel[1] as f32 - new_pixel[1] as f32,
                old_pixel[2] as f32 - new_pixel[2] as f32,
            ];

            dithered_image.put_pixel(x, y, new_pixel);

            if x + 1 < width {
                let neighbor_pixel = dithered_image.get_pixel(x + 1, y);
                let new_neighbor = [
                    (neighbor_pixel[0] as f32 + quant_error[0] * 7.0 / 16.0).max(0.0).min(255.0) as u8,
                    (neighbor_pixel[1] as f32 + quant_error[1] * 7.0 / 16.0).max(0.0).min(255.0) as u8,
                    (neighbor_pixel[2] as f32 + quant_error[2] * 7.0 / 16.0).max(0.0).min(255.0) as u8,
                    255,
                ];
                dithered_image.put_pixel(x + 1, y, Rgba(new_neighbor));
            }

            if y + 1 < height {
                if x > 0 {
                    let neighbor_pixel = dithered_image.get_pixel(x - 1, y + 1);
                    let new_neighbor = [
                        (neighbor_pixel[0] as f32 + quant_error[0] * 3.0 / 16.0).max(0.0).min(255.0) as u8,
                        (neighbor_pixel[1] as f32 + quant_error[1] * 3.0 / 16.0).max(0.0).min(255.0) as u8,
                        (neighbor_pixel[2] as f32 + quant_error[2] * 3.0 / 16.0).max(0.0).min(255.0) as u8,
                        255,
                    ];
                    dithered_image.put_pixel(x - 1, y + 1, Rgba(new_neighbor));
                }

                let neighbor_pixel = dithered_image.get_pixel(x, y + 1);
                let new_neighbor = [
                    (neighbor_pixel[0] as f32 + quant_error[0] * 5.0 / 16.0).max(0.0).min(255.0) as u8,
                    (neighbor_pixel[1] as f32 + quant_error[1] * 5.0 / 16.0).max(0.0).min(255.0) as u8,
                    (neighbor_pixel[2] as f32 + quant_error[2] * 5.0 / 16.0).max(0.0).min(255.0) as u8,
                    255,
                ];
                dithered_image.put_pixel(x, y + 1, Rgba(new_neighbor));

                if x + 1 < width {
                    let neighbor_pixel = dithered_image.get_pixel(x + 1, y + 1);
                    let new_neighbor = [
                        (neighbor_pixel[0] as f32 + quant_error[0] * 1.0 / 16.0).max(0.0).min(255.0) as u8,
                        (neighbor_pixel[1] as f32 + quant_error[1] * 1.0 / 16.0).max(0.0).min(255.0) as u8,
                        (neighbor_pixel[2] as f32 + quant_error[2] * 1.0 / 16.0).max(0.0).min(255.0) as u8,
                        255,
                    ];
                    dithered_image.put_pixel(x + 1, y + 1, Rgba(new_neighbor));
                }
            }
        }
    }
    dithered_image
}

// Encode function - Apply dithering and encode the dithered image into the default image
pub fn encode_image(secret_img: &DynamicImage, default_img: &DynamicImage) -> RgbaImage {
    let dithered_secret_img = dither_image(secret_img);

    let (width, height) = dithered_secret_img.dimensions();
    let resized_default_img = resize_image(default_img, width, height);
    let mut encoded_img = resized_default_img.to_rgba8();

    for x in 0..width {
        for y in 0..height {
            let secret_pixel = dithered_secret_img.get_pixel(x, y);
            let default_pixel = resized_default_img.get_pixel(x, y);

            let encoded_pixel = Rgba([
                (default_pixel[0] & 0xF0) | ((secret_pixel[0] & 0xF0) >> 4),
                (default_pixel[1] & 0xF0) | ((secret_pixel[1] & 0xF0) >> 4),
                (default_pixel[2] & 0xF0) | ((secret_pixel[2] & 0xF0) >> 4),
                255,
            ]);

            encoded_img.put_pixel(x, y, encoded_pixel);
        }
    }
    encoded_img
}
