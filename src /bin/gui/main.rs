use image;

fn main() {
    let img =
        image::ImageBuffer::from_pixel(100, 100, image::Rgb::from([u8::MAX, u8::MAX, u8::MAX]));
    println!("{:?}", img)
}
