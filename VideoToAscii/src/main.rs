use image::{self, DynamicImage, GenericImageView};
use rscam::{Camera, Config, Frame};
use std::{env, process};

use std::thread;
use std::time::Duration;

const PIX_BITS: usize = 4;

fn get_ascii(intensity: u8) -> &'static str {
    let mut ascii = [
        "#", "B", "A", "C", "(", "g", "a", "v", "r", "i", "+", "=", "*", "^", "-", "_", ",", ".",
        " ",
    ];
    let map_left = 1;
    let index = intensity
        / ((255 / (ascii.len() as u8 - map_left)) + (255 % (ascii.len() as u8 - map_left)));

    ascii.reverse();
    return ascii[index as usize];
}

fn draw_ascii_image(image: &DynamicImage) {
    let (width, height) = image.dimensions();

    let scale = height / 200;

    for y in 0..height {
        for x in 0..width {
            if y % (scale * 2) == 0 && x % (scale * 2) == 0 {
                let pix = image.get_pixel(x, y);
                let intensity = calculate_intensity((&pix[0], &pix[1], &pix[2], &pix[3]));
                print!("{}", get_ascii(intensity));
            }
        }
        if y % (scale * 2) == 0 {
            println!();
        }
    }
}

fn calculate_intensity(pix: (&u8, &u8, &u8, &u8)) -> u8 {
    // This is avegaging technique
    // return *pix.0/3 + *pix.1/3 + *pix.2/3;
    //

    //
    // This considers physical brightness of colors
    //
    let gottem = *pix.0 as f32 * *pix.0 as f32 * 0.241
        + *pix.1 as f32 * *pix.1 as f32 * 0.691
        + *pix.2 as f32 * *pix.2 as f32 * 0.068;
    return gottem.sqrt() as u8;
}

fn draw_ascii_video(frame: &Frame) {
    let (_, width) = frame.resolution;

    let scale = 2;

    let mut index = 0;
    let mut y: u32 = 0;
    while index < frame.len() {
        let pixel_index = index / PIX_BITS;
        let x = pixel_index as u32 % width;
        if x == 0 {
            y = (pixel_index as u32 - x) / width;
        }
        if y % (scale * 2) == 0 && x % (scale / 2) == 0 {
            let intensity = calculate_intensity((
                &frame[index],
                &frame[index + 1],
                &frame[index + 2],
                &frame[index + 3],
            ));
            print!("{}", get_ascii(intensity));
        }
        index = index + 4;
        if y % (scale * 2) == 0 && x == width - 1 {
            println!();
        }
    }
}

fn use_camera(source: &String) {
    let camera_result = Camera::new(source);
    let mut camera = match camera_result {
        Ok(camera) => camera,
        Err(..) => {
            println!("Error opening webcam");
            process::exit(1);
        }
    };

    camera
        .start(&Config {
            interval: (1, 30), // 30 fps.
            resolution: (640, 480),
            format: b"RGB3",
            ..Default::default()
        })
        .unwrap();

    loop {
        let frame_result = camera.capture();
        let frame = match frame_result {
            Ok(frame) => frame,
            Err(..) => {
                println!("Error reading frame");
                process::exit(1)
            }
        };
        print!("\x1B[2J\x1B[1;1H");
        draw_ascii_video(&frame);
        thread::sleep(Duration::from_millis(34));
    }
}

fn use_image(source: &String) {
    let image = image::open(source).unwrap();
    draw_ascii_image(&image);
}

fn help() {
    println!("Usage: ./asciiman <options>");
    println!("Options:");
    println!("\t--video /dev/video0");
    println!("\t--image /path/to/image");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("No source args passed.");
            help()
        }
        3 => {
            let option = &args[1];
            match &option[..] {
                "--video" => use_camera(&args[2]),
                "--image" => use_image(&args[2]),
                _ => help(),
            }
        }
        _ => {
            println!("Invalid usage");
            help()
        }
    }
}
