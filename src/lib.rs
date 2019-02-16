#[macro_use] extern crate serde_derive;

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
            .map(|obj| obj.check_hit(ray))
            .min_by(|r1,r2|
                r1.clone()
                    .map(|r| r.at)
                    .partial_cmp(&r2.clone().map(|r| r.at))
                    .unwrap_or(std::cmp::Ordering::Equal))
            .and_then(|v| v)
    }

    fn calculate_ray(&self, mut ray: Ray) -> Color {
        let mut radiance = Color::black();
        let mut weight = 0.5;

        while let Some(record) = self.get_hit_point(&ray) {
            radiance += record.object.lambertian.scale(weight);
            let iflux = record.object.incident_flux(record.point);
            weight *= record.object.bsdf() * iflux.direction.dot(record.normal) / record.object.flux_prob(record.normal, &iflux);

            let roulette_threshold = 0.5;
            if Scene::rossian_roulette(roulette_threshold) {
                return radiance;
            }

            ray = iflux;
            weight *= roulette_threshold;
        }

        radiance
    }

    fn render(&self) -> Vec<Color> {
        let fov: f32 = 90.0;
        let mut pixels = vec![Color::black(); (self.width * self.height) as usize];
        let pixel_array = pixels.as_mut_slice();

        let from = V3(0.0, 0.0, -1.0 / 2.0 / (fov / 2.0).tan());

        for j in 0..self.height {
            for i in 0..self.width {
                for _ in 0..self.samples_per_pixel {
                    let point_in_picture = V3(i as f32 / self.width as f32 - 0.5, j as f32 / self.width as f32 - (self.height as f32 / self.width as f32 / 2.0), 0.0);

                    pixel_array[(i + j * self.width) as usize] += self.calculate_ray(Ray {
                        origin: from,
                        direction: V3U::from_v3(point_in_picture - from),
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

