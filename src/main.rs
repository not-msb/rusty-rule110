use std::{fs::{File, self}, io::{Error, Write}};

const W: usize   = 20;
const H: usize   = 20;
const PATH: &str = "image.ppm";

fn main() -> Result<(), Error> {
    let mut image = [0; W * H];

    if File::open(PATH).is_ok() {
        fs::remove_file(PATH)?;
    }
    let mut file = File::create(PATH)?;

    let header = format!("P4 {} {}", W, H);
    file.write_all(header.as_bytes())?;
    
    gen(&mut image);

    file.write_all(&image)?;

    Ok(())
}

fn gen(image: &mut [u8; W * H]) {
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
        }
    }
}
