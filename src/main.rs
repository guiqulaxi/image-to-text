extern crate image;
extern crate imageproc;
use std::env;
use std::path::Path;
use std::cmp;
fn main() {


    let mut max_len = 80;
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("usage: iamge_to_text filename max_length ");
    }
    let path = Path::new(&args[1]);
    if args.len() > 2 {
        max_len = args[2].parse::<u32>().unwrap();
    }
    let im = image::open(&path);
    if im.is_err() {
        println!("open file {}", args[1]);
        return;
    }
    let mut gray_im = im.unwrap().to_luma();
    gray_im = imageproc::contrast::equalize_histogram(&gray_im);
    gray_im = imageproc::contrast::threshold(&gray_im, 128);
    let (width, height) = gray_im.dimensions();
    let mut resize_time = 1f32;

    let len = cmp::max(width, height);
    if len > max_len {
        resize_time = len as f32 / (max_len as f32);
    }
    let resized_width = (width as f32 / resize_time) as u32;
    let resized_height = (height as f32 / resize_time) as u32;
    let resized_im = image::imageops::resize(
        &gray_im,
        resized_width,
        resized_height,
        image::FilterType::Lanczos3,
    );
    let data = resized_im.into_raw();

    for y in 0..resized_height {
        for x in 0..resized_width {
            let p = data[(y * resized_width + x) as usize];

            if p > 128 {
                print!(" ");
            } else {
                //
                print!("*");
            }
        }
        print!("\n");
    }
}
