#[macro_use] extern crate serde_derive;
extern crate quickcheck;
#[macro_use(quickcheck)] extern crate quickcheck_macros;

use rayon::prelude::*;

pub mod vector;
pub mod color;
pub mod object;

use vector::*;
use color::*;
use object::*;
use std::io::{self, BufWriter, Write};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct SceneData {
    width: i32,
    height: i32,
    samples_per_pixel: i32,
    objects: Vec<Object>,
}

pub struct Scene {
    width: i32,
    height: i32,
    samples_per_pixel: i32,
    objects: Vec<Object>,
    lights: Vec<usize>,
}

impl Scene {
    pub fn new(data: SceneData) -> Scene {
        let lights = data.objects.iter().enumerate().filter(|(_,obj)| obj.emission != Color::black()).map(|p| p.0).collect();

        Scene {
            width: data.width,
            height: data.height,
            samples_per_pixel: data.samples_per_pixel,
            objects: data.objects,
            lights: lights,
        }
    }

    pub fn write(&self, file_path: &str) -> io::Result<()> {
        let pixels = self.render();

        self.write_ppm(file_path, pixels)
    }

    fn pick_random_light(&self) -> &Object {
        let r = rand::random::<usize>();
        let i = self.lights[r % self.lights.len()];
        &self.objects[i]
    }

    fn rossian_roulette(threshold: f32) -> bool {
        rand::random::<f32>() > threshold
    }

    fn get_hit_point(&self, ray: &Ray) -> Option<HitRecord> {
        self.objects.iter()
            .flat_map(|obj| obj.check_hit(ray))
            .min_by(|r1,r2| r1.at.partial_cmp(&r2.at).unwrap_or(std::cmp::Ordering::Equal))
    }

    fn is_transported(&self, ray: &Ray, tmax: f32) -> bool {
        if let Some(record) = self.get_hit_point(ray) {
            if record.at < tmax {
                return true;
            }
        }

        false
    }

    fn radiance(&self, record: HitRecord, depth: i32) -> Color {
        let roulette_threshold =
            if depth <= 5 { 1.0 }
            else if depth < 64 { record.object.color.as_v3().0.max(record.object.color.as_v3().1.max(record.object.color.as_v3().2)) }
            else { record.object.color.as_v3().0.max(record.object.color.as_v3().1.max(record.object.color.as_v3().2)) * 0.5f32.powi(depth - 64) };
        if Scene::rossian_roulette(roulette_threshold) {
            return record.object.emission;
        }

        let mut radiance = Color::black();

        // Next Event Estimation
        let light_object = self.pick_random_light();
        let light_point = {
            let theta = 2.0 * std::f32::consts::PI * rand::random::<f32>();
            let phi = 2.0 * std::f32::consts::PI * rand::random::<f32>();

            let p = V3(
                theta.cos() * phi.sin(),
                theta.cos() * phi.cos(),
                theta.sin(),
            );
            
            p.scale(light_object.radius) + light_object.center
        };
        let light_distance = light_point - record.point;
        let shadow_ray = Ray {
            origin: record.point,
            direction: V3U::from_v3(light_distance),
        };
        if self.is_transported(&shadow_ray, light_distance.norm() - 0.001) {
            //radiance += record.object.color.blend(light_object.emission).scale(shadow_ray.direction.dot(record.normal).abs() / light_distance.norm());
        }

        let iflux = Object::incident_flux(record.normal);
        let ray = Ray {
            origin: record.point,
            direction: iflux,
        };

        // radiance calculation
        if let Some(record) = self.get_hit_point(&ray) {
            let weight = record.object.color.scale(1.0 / roulette_threshold);
            radiance += record.object.emission + self.radiance(record, depth + 1).blend(weight);
        }

        radiance
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
        let world_screen = (30.0 * self.width as f32 / self.height as f32, 30.0);
        let camera_position = V3(50.0, 52.0, 120.0);
        let camera_dir = V3U::from_v3(V3(0.0, -0.04, -1.0));
        let camera_up = V3U::unsafe_new(0.0, 1.0, 0.0);
        
        let screen_dist = 40.0;

        let ux = camera_dir.cross(camera_up);
        let uy = ux.cross(camera_dir);

        (0..self.width * self.height).into_par_iter().map(move |k: i32| {
            let j = k / self.width;
            let i = k % self.width;

            (0..self.samples_per_pixel).map(|_: i32| {
                let point_in_picture
                    = camera_position
                    + camera_dir.as_v3().scale(screen_dist)
                    - ux.as_v3().scale(world_screen.0 * (1.0 - (2.0 * i as f32 + rand::random::<f32>()) / self.width as f32))
                    + uy.as_v3().scale(world_screen.1 * (1.0 - (2.0 * j as f32 + rand::random::<f32>()) / self.height as f32));

                self.calculate_ray(Ray {
                    origin: camera_position,
                    direction: V3U::from_v3(point_in_picture - camera_position),
                })
            }).fold(Color::black(), |c1,c2| c1 + c2).scale(1.0 / self.samples_per_pixel as f32)
        }).collect::<Vec<_>>()
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

