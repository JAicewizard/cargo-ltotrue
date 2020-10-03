// Copyright 2020 Antipy V.O.F. All rights reserved.
extern crate martens_martens_icons_lib;

use image;

fn main() {
    let img =
        image::ImageBuffer::from_pixel(100, 100, image::Rgb::from([u8::MAX, u8::MAX, u8::MAX]));
    println!("{:?}", img)
}
