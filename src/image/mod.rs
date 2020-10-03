// Copyright 2020 Antipy V.O.F. All rights reserved.
use image;
use image::Rgb;

pub struct CharBlock {
    pub x: u32,
    pub y: u32,
}

pub fn load_image(
    img: &image::RgbImage,
    block_size: CharBlock,
) -> image::ImageBuffer<Rgb<u16>, Vec<u16>> {
    let block_count_x: u32 = img.width() / block_size.x;
    let block_count_y: u32 = img.height() / block_size.y;

    image::ImageBuffer::<Rgb<u16>, Vec<u16>>::new(block_count_x, block_count_y)
}
