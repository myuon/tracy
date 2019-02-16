#[macro_use] extern crate serde_derive;
extern crate quickcheck;
#[macro_use(quickcheck)] extern crate quickcheck_macros;

pub mod vector;
pub mod color;
pub mod object;

use vector::*;
use color::*;
use object::*;
use std::io::{self, BufWriter, Write};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Scene {
    width: i32,
    height: i32,
    samples_per_pixel: i32,
    objects: Vec<Object>,
}

impl Scene {
    pub fn write(&self, file_path: &str) -> io::Result<()> {
        let pixels = self.render();

        self.write_ppm(file_path, pixels)
    }

    fn rossian_roulette(threshold: f32) -> bool {
        rand::random::<f32>() > threshold
    }

    fn get_hit_point(&self, ray: &Ray) -> Option<HitRecord> {
        self.objects.iter()
            .flat_map(|obj| obj.check_hit(ray))
            .min_by(|r1,r2| r1.at.partial_cmp(&r2.at).unwrap_or(std::cmp::Ordering::Equal))
    }

    fn radiance(&self, record: HitRecord, depth: i32) -> Color {
        let roulette_threshold = 0.5;
        if 5 < depth && depth < 15 && Scene::rossian_roulette(roulette_threshold) {
            return record.object.emission;
        }

        let iflux = Object::incident_flux(record.normal);
        let ray = Ray {
            origin: record.point,
            direction: iflux,
        };

        if let Some(record) = self.get_hit_point(&ray) {
            return record.object.emission +
                record.object.color.blend(self.radiance(record, depth + 1)).scale(1.0 / roulette_threshold);
        } else {
            return Color::black();
        }
    }

    fn calculate_ray(&self, ray: Ray) -> Color {
        if let Some(record) = self.get_hit_point(&ray) {
            self.radiance(record, 0)
        } else {
            Color::black()
        }
    }

    fn render(&self) -> Vec<Color> {
        let fov: f32 = 90.0;
        let mut pixels = vec![Color::black(); (self.width * self.height) as usize];
        let pixel_array = pixels.as_mut_slice();
        let world_screen = (30.0 * self.width as f32 / self.height as f32, 30.0);
        let camera_position = V3(50.0, 52.0, 220.0);
        let camera_dir = V3U::from_v3(V3(0.0, -0.04, -1.0));
        let camera_up = V3U::unsafe_new(0.0, 1.0, 0.0);
        
        let screen_dist = 40.0;

        let ux = camera_dir.cross(camera_up);
        let uy = ux.cross(camera_dir);

        for j in 0..self.height {
            for i in 0..self.width {
                for _ in 0..self.samples_per_pixel {
                    let point_in_picture
                        = camera_position
                        + camera_dir.as_v3().scale(screen_dist)
                        - ux.as_v3().scale(world_screen.0 * (1.0 - (2.0 * i as f32 + rand::random::<f32>()) / self.width as f32))
                        + uy.as_v3().scale(world_screen.1 * (1.0 - (2.0 * j as f32 + rand::random::<f32>()) / self.height as f32));

                    pixel_array[(i + j * self.width) as usize] += self.calculate_ray(Ray {
                        origin: camera_position,
                        direction: V3U::from_v3(point_in_picture - camera_position),
                    });
                }

                pixel_array[(i + j * self.width) as usize] /= self.samples_per_pixel as f32;
            }
        }

        pixels
    }

    fn write_ppm(&self, file_path: &str, pixels: Vec<Color>) -> io::Result<()> {
        let mut file = BufWriter::new(fs::File::create(file_path).unwrap());
        file.write(format!("P3\n{} {}\n255\n", self.width, self.height).as_bytes())?;

        let pixel_array = pixels.as_slice();

        for j in 0..self.height {
            for i in 0..self.width {
                let c = pixel_array[(i + j * self.width) as usize].gamma_correction(2.2).nan_safe();

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

