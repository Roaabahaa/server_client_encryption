use tokio::net::UdpSocket;
use image::{DynamicImage, ImageFormat};
use std::env;
use crate::middleware::decode_image;

mod middleware;

#[tokio::main]
async fn main() {
    // Get server IP, port, and image paths from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: client <server_ip> <server_port> <secret_image_path> <default_image_path>");
        return;
    }
    let server_ip = &args[1];
    let server_port = &args[2];
    let secret_image_path = &args[3];
    let default_image_path = &args[4];

    // Create a UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0").await.expect("Could not bind client socket");

    // Load secret and default images
    let secret_img = DynamicImage::open(secret_image_path).expect("Failed to open secret image");
    let default_img = DynamicImage::open(default_image_path).expect("Failed to open default image");

    // Convert images to bytes
    let mut secret_img_bytes = vec![];
    let mut default_img_bytes = vec![];
    secret_img.write_to(&mut secret_img_bytes, ImageFormat::Png).expect("Failed to write secret image to bytes");
    default_img.write_to(&mut default_img_bytes, ImageFormat::Png).expect("Failed to write default image to bytes");

    // Send both images to the server
    let server_address = format!("{}:{}", server_ip, server_port);
    socket.send_to(&[secret_img_bytes, default_img_bytes].concat(), &server_address)
          .await.expect("Failed to send images to server");

    // Receive encoded image back from the server
    let mut buffer = [0u8; 65_536]; // Buffer for the incoming encoded image
    let (len, _) = socket.recv_from(&mut buffer).await.expect("Failed to receive encoded image from server");

    // Decode received encoded image
    let encoded_img = image::load_from_memory(&buffer[..len]).expect("Failed to load received encoded image");
    let decoded_img = decode_image(&encoded_img);
    decoded_img.save("decoded_secret_image.png").expect("Failed to save decoded image");

    println!("Decoded image saved as decoded_secret_image.png");
}
