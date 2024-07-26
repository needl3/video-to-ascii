use image::{self, DynamicImage, GenericImageView};
use rscam::{Camera, Config, Frame};
use std::{env, process};

use std::thread;
use std::time::Duration;

const PIX_BITS: usize = 4;

fn get_ascii(intensity: u8) -> &'static str {
    let ascii = [
        " ", ".", "~", "+", "=", "r", "c", "a", "g", "x", "C", "A", "B", "M", "%", "$", "#",
    ];
    let index = intensity / ((255 / ascii.len() as u8) + (255 % ascii.len() as u8));

    return ascii[index as usize];
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

fn draw_ascii_image(image: &DynamicImage, color: i8) {
    let (width, height) = image.dimensions();

    let scale = height / 200;

    let mut to_print = String::new();

    for y in 0..height {
        for x in 0..width {
            if y % (scale * 2) == 0 && x % (scale * 2) == 0 {
                let pix = image.get_pixel(x, y);
                if color > 0 {
                    to_print += &format!("\x1B[48;2;{};{};{}m ", pix[0], pix[1], pix[2],);
                } else {
                    let intensity = calculate_intensity((&pix[0], &pix[1], &pix[2], &pix[3]));
                    if color > -1 {
                        to_print += &format!(
                            "\x1B[38;2;{};{};{}m{}",
                            pix[0],
                            pix[1],
                            pix[2],
                            get_ascii(intensity)
                        );
                    } else {
                        to_print += get_ascii(intensity);
                    }
                }
            }
        }

        if y % (scale * 2) == 0 {
            if color > 0 {
                to_print += "\x1B[38;2;255;255;255;48;2;0;0;0m";
            } else if color > -1 {
                to_print += "\x1B[38;2;255;255;255m";
            }
            to_print += "\n";
        }
    }
    print!("{}", to_print);
}

fn use_image(source: &String, color: i8) {
    let image = image::open(source).unwrap();
    draw_ascii_image(&image, color);
}

fn draw_ascii_video(frame: &Frame, color: i8) {
    //
    // color: -1 = no color, 0 = text only, 1 = text and background color
    //
    let (_, width) = frame.resolution;

    let scale = 2;

    let mut index = 0;
    let mut y: u32 = 0;
    let mut to_print = String::new();
    while index < frame.len() {
        let pixel_index = index / PIX_BITS;
        let x = pixel_index as u32 % width;
        if x == 0 {
            y = pixel_index as u32 / width;
        }
        if y % (scale * 2) == 0 && x % (scale / 2) == 0 {
            if color > 0 {
                to_print += &format!(
                    "\x1B[48;2;{};{};{}m ",
                    frame[index],
                    frame[index + 1],
                    frame[index + 2],
                );
            } else {
                let intensity = calculate_intensity((
                    &frame[index],
                    &frame[index + 1],
                    &frame[index + 2],
                    &frame[index + 3],
                ));

                if color > -1 {
                    to_print += &format!(
                        "\x1B[38;2;{};{};{}m{}",
                        frame[index],
                        frame[index + 1],
                        frame[index + 2],
                        get_ascii(intensity)
                    );
                } else {
                    to_print += get_ascii(intensity);
                }
            }
            if x == width - 1 {
                if color > 0 {
                    to_print += "\x1B[38;2;255;255;255;48;2;0;0;0m";
                } else if color > -1 {
                    to_print += "\x1B[38;2;255;255;255m";
                }
                to_print += "\n";
            }
        }
        index = index + 4;
    }
    print!("{}", to_print);
}

fn use_camera(source: &String, color: i8) {
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
        draw_ascii_video(&frame, color);
        thread::sleep(Duration::from_millis(20)); // Reqd only for --color option because of my
                                                  // lappy of quantum computing speed
    }
}

fn help() {
    println!("Usage: ./asciiman <options>");
    println!("Options:");
    println!("\t--video /dev/video0");
    println!("\t--image /path/to/image");
    println!("\t\t--color <options>");
    println!("Color options:");
    println!("No options: Only use foreground colors");
    println!("bgcolor : Use background color");
    println!("Note: Omit --color to not use colored rendering");
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
                "--video" => use_camera(&args[2], -1),
                "--image" => use_image(&args[2], -1),
                _ => help(),
            }
        }
        4 => {
            if args[3] != "--color" {
                help();
                return;
            }
            println!("\n\n NOTE: Colored Rendering will be choppy");
            thread::sleep(Duration::from_millis(1000));

            let option = &args[1];
            match &option[..] {
                "--video" => use_camera(&args[2], 0),
                "--image" => use_image(&args[2], 0),
                _ => help(),
            }
        }
        5 => {
            if args[3] != "--color" {
                help();
                return;
            }
            println!("\n\nNOTE: Colored Rendering will be the choppiest");
            thread::sleep(Duration::from_millis(1000));

            let option = &args[1];
            match &option[..] {
                "--video" => use_camera(&args[2], 1),
                "--image" => use_image(&args[2], 1),
                _ => help(),
            }
        }
        _ => {
            println!("Invalid usage");
            help()
        }
    }
}
