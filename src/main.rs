use std::{io, error::Error};
use image::{GenericImage, GenericImageView, ImageBuffer, DynamicImage, ImageError};
fn main() -> Result<(), Box<dyn Error>>{
    let stdin = io::stdin();
    println!("請輸入圖檔輸入路徑：");
    let mut input_path = String::new();
    stdin.read_line(&mut input_path)?;
    println!("請輸入圖檔輸出路徑：");
    let mut output_path = String::new();
    stdin.read_line(&mut output_path)?;
    println!("請輸入圖片寬度比：");
    let mut width_ratio = String::new();
    stdin.read_line(&mut width_ratio)?;
    println!("請輸入圖片長度比：");
    let mut height_ratio = String::new();
    stdin.read_line(&mut height_ratio)?;
    println!("請輸入圖片邊緣比：");
    let mut padding_ratio = String::new();
    stdin.read_line(&mut padding_ratio)?;
    let img = image::open(&input_path.trim())?;
    let padded = add_padding(img, (width_ratio.trim().parse()?, height_ratio.trim().parse()?, padding_ratio.trim().parse()?))?;
    let img_buf = gen_grid(DynamicImage::ImageRgba8(padded), 7, 7, 1)?;
    img_buf.save(&output_path.trim())?;
    Ok(())
}

fn add_padding(img: DynamicImage, (width_ratio, height_ratio, padding_ratio): (u32, u32, u32)) -> Result<ImageBuffer<image::Rgba<u8>, Vec<u8>>, ImageError> {
    let (width, height) = img.dimensions();
    let mut img_buf: ImageBuffer<image::Rgba<u8>, Vec<u8>>;
    // if width / width_ratio >= height / height_ratio
    if width * height_ratio >= height * width_ratio {
        let new_height = width * height_ratio / width_ratio;
        img_buf = image::ImageBuffer::new(width, new_height);
        for y in 0..new_height {
            for x in 0..width {
                let pixel = img_buf.get_pixel_mut(x, y);
                let image::Rgba(_) = *pixel;
                *pixel = image::Rgba([255, 255, 255, 255]);
            }
        }
        img_buf.copy_from(&img, 0, (new_height - height) / 2)?;
    } else {
        let new_width = height * width_ratio / height_ratio;
        img_buf = image::ImageBuffer::new(new_width, height);
        for y in 0..height {
            for x in 0..new_width {
                let pixel = img_buf.get_pixel_mut(x, y);
                let image::Rgba(_) = *pixel;
                *pixel = image::Rgba([255, 255, 255, 255]);
            }
        }
        img_buf.copy_from(&img, (new_width - width) / 2, 0)?;
    }
    let padding = height * padding_ratio / height_ratio;
    let new_width = img_buf.dimensions().0 + padding * 2;
    let new_height = img_buf.dimensions().1 + padding * 2;
    let mut img_buf2 = image::ImageBuffer::new(new_width, new_height);
    for y in 0..new_height {
        for x in 0..new_width {
            let pixel = img_buf2.get_pixel_mut(x, y);
            let image::Rgba(_) = *pixel;
            *pixel = image::Rgba([255, 255, 255, 255]);
        }
    }
    img_buf2.copy_from(&img_buf, padding, padding)?;
    Ok(img_buf2)
}

fn gen_grid(img: DynamicImage, cols: u32, rows: u32, line_thickness: u32) -> Result<ImageBuffer<image::Rgba<u8>, Vec<u8>>, ImageError> {
    let (width, height) = img.dimensions();
    let new_width = width * cols + line_thickness * (cols + 1);
    let new_height = height * rows + line_thickness * (rows + 1);
    let mut img_buf = image::ImageBuffer::new(new_width, new_height);
    for row in 0..rows {
        img_buf = draw_horizontal_line(img_buf, row * (line_thickness + height), line_thickness)?;
        for col in 0..cols {
            img_buf.copy_from(&img, line_thickness + col * (line_thickness + width), line_thickness + row * (line_thickness + height))?;
        }
    }
    for col in 0..cols {
        img_buf = draw_vertical_line(img_buf, col * (line_thickness + width), line_thickness)?;
    }
    Ok(img_buf)
}

fn draw_horizontal_line(mut img_buf: ImageBuffer<image::Rgba<u8>, Vec<u8>>, y: u32, line_thickness: u32) -> Result<ImageBuffer<image::Rgba<u8>, Vec<u8>>, ImageError> {
    for y in y..(y + line_thickness) {
        for x in 0..(img_buf.dimensions().0) {
            let pixel = img_buf.get_pixel_mut(x, y);
            let image::Rgba(_) = *pixel;
            *pixel = image::Rgba([0, 0, 0, 255]);
        }
    }
    Ok(img_buf)
}

fn draw_vertical_line(mut img_buf: ImageBuffer<image::Rgba<u8>, Vec<u8>>, x: u32, line_thickness: u32) -> Result<ImageBuffer<image::Rgba<u8>, Vec<u8>>, ImageError> {
    for x in x..(x + line_thickness) {
        for y in 0..(img_buf.dimensions().1) {
            let pixel = img_buf.get_pixel_mut(x, y);
            let image::Rgba(_) = *pixel;
            *pixel = image::Rgba([0, 0, 0, 255]);
        }
    }
    Ok(img_buf)
}