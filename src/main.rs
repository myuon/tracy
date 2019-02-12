use std::io::{self, BufWriter, Write};
use std::fs;

#[derive(Clone, Copy)]
struct Color(u8, u8, u8);

impl Color {
    fn red(&self) -> u8 {
        self.0
    }

    fn green(&self) -> u8 {
        self.1
    }

    fn blue(&self) -> u8 {
        self.2
    }
}

struct Screen {
    width: i32,
    height: i32,
}

impl Screen {
    fn write(&self, file_path: &str) -> io::Result<()> {
        let pixels = self.calculate();

        self.write_ppm(file_path, pixels)
    }

    fn calculate(&self) -> Vec<Color> {
        let mut pixels = vec![Color(255,255,255); (self.width * self.height) as usize];

        pixels
    }

    fn write_ppm(&self, file_path: &str, pixels: Vec<Color>) -> io::Result<()> {
        let mut file = BufWriter::new(fs::File::create(file_path).unwrap());
        file.write(format!("P3\n{} {}\n255\n", self.width, self.height).as_bytes())?;

        let pixel_array = pixels.as_slice();

        for j in 0..self.height {
            for i in 0..self.width {
                let c = pixel_array[(i + j * self.width) as usize];

                file.write(format!(
                    "{} {} {}\n",
                    c.red(),
                    c.green(),
                    c.blue(),
                ).as_bytes())?;
            }
        }

        Ok(())
    }
}

fn main() {
    let screen = Screen{
        width: 200,
        height: 150
    };

    screen.write("./out/image.ppm").unwrap();
}
