use image::{DynamicImage, RgbaImage, Rgba};

// Decode function - Extract the embedded 4 bits and expand them to the full color range
pub fn decode_image(encoded_img: &DynamicImage) -> RgbaImage {
    let (width, height) = encoded_img.dimensions();
    let mut secret_img = RgbaImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let encoded_pixel = encoded_img.get_pixel(x, y);

            let decoded_pixel = Rgba([
                (encoded_pixel[0] & 0x0F) << 4,
                (encoded_pixel[1] & 0x0F) << 4,
                (encoded_pixel[2] & 0x0F) << 4,
                255,
            ]);

            secret_img.put_pixel(x, y, decoded_pixel);
        }
    }
    secret_img
}
