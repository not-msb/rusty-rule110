use std::{fs, io::Error, path::Path};

use image::{ImageBuffer, RgbImage, Rgb};

const W: usize   = 30;
const H: usize   = 30;

fn main() -> Result<(), Error> {
    let mut image = [0; W * H];
    image[W-1] = 1;
    
    if Path::exists(Path::new("tmp")) {
        fs::remove_dir("tmp")?;
    }
    fs::create_dir("tmp")?;
 
    gen(&mut image)?;

    Ok(())
}

fn gen(image: &mut [u8; W * H]) -> Result<(), Error> {
    let mut fi = 0;

    for i in 0..image.len()-W-1 {
        let p1 = char::from_digit(image[i] as u32, 10).unwrap();
        let p2 = char::from_digit(image[i+1] as u32, 10).unwrap();
        let p3 = char::from_digit(image[i+2] as u32, 10).unwrap();

        let old_pattern = format!("{}{}{}", p1, p2, p3);

        let w = if i >= W*H-W-1 {
            i-(W*H-W-1)
        } else {
            i+W+1
        };

        image[w] = match old_pattern.as_str() {
            "111" => 0,
            "110" => 1,
            "101" => 1,
            "100" => 0,
            "011" => 1,
            "010" => 1,
            "001" => 1,
            "000" => 0,
            _ => 0
        };
        if image[w] == 1 {
            write_png(&format!("image-{}.png", fi), image)?;
            fi += 1;
        }
    }

    Ok(())
}

fn write_png(path: &str, data: &[u8]) -> Result<(), Error> {
    let file_path = "tmp/".to_string() + path;
    if Path::exists(Path::new(&file_path)) {
        fs::remove_file(&file_path)?;
    }
    let mut image: RgbImage = ImageBuffer::new(W as u32, H as u32);

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let pos = x + y * W as u32;
        *pixel = match data[pos as usize] {
            1 => Rgb([255, 0, 0]),
            0 => Rgb([0, 0, 0]),
            _ => Rgb([0, 0, 0])
        };
    }

    image.save(&file_path).unwrap();

    Ok(())
}