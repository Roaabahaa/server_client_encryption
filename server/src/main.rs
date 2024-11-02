use tokio::net::UdpSocket;
use image::{DynamicImage, ImageFormat, load_from_memory};
use std::env;
use std::io::Cursor;
use crate::middleware::{encode_image, load_image};

mod middleware;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: server <port>");
        return;
    }
    let port = &args[1];

    let socket = UdpSocket::bind(format!("0.0.0.0:{}", port)).await.expect("Could not bind server");
    println!("Server listening on UDP port {}", port);

    let mut buffer = [0u8; 65_536]; // 64KB buffer for image data

    loop {
        let (len, client_addr) = socket.recv_from(&mut buffer).await.expect("Failed to receive data");
        println!("Received data from client: {}", client_addr);

        // Load images from received bytes (assuming half is secret and half is default image)
        let secret_img = load_from_memory(&buffer[..len / 2]).expect("Failed to load secret image");
        let default_img = load_from_memory(&buffer[len / 2..len]).expect("Failed to load default image");

        // Encode the secret image into the default image
        let encoded_img = encode_image(&secret_img, &default_img);

        // Convert encoded image to bytes to send back to the client
        let mut encoded_img_data = Vec::new();
        let mut cursor = Cursor::new(&mut encoded_img_data);
        encoded_img.write_to(&mut cursor, ImageFormat::Png).expect("Failed to encode image to PNG format");

        // Send encoded image data back to the client
        socket.send_to(&encoded_img_data, &client_addr).await.expect("Failed to send encoded image to client");
        println!("Encoded image sent back to client at {}", client_addr);
    }
}
