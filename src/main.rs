extern crate rand;
#[macro_use] extern crate serde_derive;
extern crate serde_yaml;
extern crate failure;

use std::io::{self, BufWriter, Write, BufReader};
use std::fs;
use std::ops::{AddAssign, DivAssign, Sub, Add};

#[derive(Clone, Copy, Serialize, Deserialize)]
struct V3(f32,f32,f32);

impl V3 {
    fn zero() -> V3 {
        V3(0.0, 0.0, 0.0)
    }

    fn dot(self, other: V3) -> f32 {
        self.0 * other.0 +
        self.1 * other.1 +
        self.2 * other.2
    }

    fn square_norm(&self) -> f32 {
        self.dot(self.clone())
    }

    fn norm(&self) -> f32 {
        self.square_norm().sqrt()
    }

    fn scale(self, k: f32) -> V3 {
        V3(
            self.0 * k,
            self.1 * k,
            self.2 * k,
        )
    }
}

impl Add for V3 {
    type Output = V3;

    fn add(self, other: V3) -> V3 {
        V3(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
        )
    }
}

impl Sub for V3 {
    type Output = V3;

    fn sub(self, other: V3) -> V3 {
        V3(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
struct Color(f32,f32,f32);

impl Color {
    fn red(&self) -> u8 {
        (self.0 * 255.0) as u8
    }

    fn green(&self) -> u8 {
        (self.1 * 255.0) as u8
    }

    fn blue(&self) -> u8 {
        (self.2 * 255.0) as u8
    }

    fn scale(self, k: f32) -> Color {
        Color(
            self.0 * k,
            self.1 * k,
            self.2 * k,
        )
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        *self = Color(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
        )
    }
}

impl DivAssign<f32> for Color {
    fn div_assign(&mut self, other: f32) {
        *self = Color(
            self.0 / other,
            self.1 / other,
            self.2 / other,
        )
    }
}

#[derive(Serialize, Deserialize)]
struct Object {
    center: V3,
    radius: f32,
    lambertian: Color,
}

#[derive(Clone)]
struct HitRecord<'a> {
    at: f32,
    point: V3,
    normal: V3,
    object: &'a Object,
}

impl Object {
    fn check_hit(&self, ray: &Ray) -> Option<HitRecord> {
        let b = ray.direction.dot(ray.origin - self.center);
        let c = (ray.origin - self.center).square_norm() - self.radius*self.radius;
        let discriminant = b * b - c;

        if discriminant > 0.0 {
            let t = -b - discriminant.sqrt();
            if t > 0.0 {
                return Some(HitRecord{
                    at: t,
                    point: ray.extend_at(t),
                    normal: ray.extend_at(t) - self.center,
                    object: self,
                });
            }

            let t = -b + discriminant.sqrt();
            if t > 0.0 {
                return Some(HitRecord{
                    at: t,
                    point: ray.extend_at(t),
                    normal: ray.extend_at(t) - self.center,
                    object: self,
                });
            }
        }

        None
    }

    fn bsdf(&self) -> f32 {
        1.0 / std::f32::consts::PI
    }

    fn incident_flux(&self, at: V3) -> Ray {
        loop {
            let p = V3(rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()).scale(2.0) - V3(1.0, 1.0, 1.0);
            if p.square_norm() < 1.0 {
                return Ray {
                    origin: at,
                    direction: p,
                };
            }
        }        
    }

    fn flux_prob(&self, normal: V3, ray: Ray) -> f32 {
        normal.dot(ray.direction) / normal.norm() / ray.direction.norm() / std::f32::consts::PI
    }
}

struct Ray {
    origin: V3,
    direction: V3,
}

impl Ray {
    fn extend_at(&self, k: f32) -> V3 {
        self.origin + self.direction.scale(k)
    }
}

#[derive(Serialize, Deserialize)]
struct Scene {
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

    fn calculate_ray(&self, ray: Ray) -> Color {
        let mut radiance = Color(0.0, 0.0, 0.0);
        let mut weight = 0.5;

        while let Some(record) = self.get_hit_point(&ray) {
            radiance += record.object.lambertian.scale(weight);
            let iflux = record.object.incident_flux(record.point);
            weight *= record.object.bsdf() * iflux.direction.dot(record.normal) / record.object.flux_prob(record.normal, iflux);

            let roulette_threshold = 0.5;
            if !Scene::rossian_roulette(roulette_threshold) {
                return radiance;
            }

            weight *= roulette_threshold;
        }

        radiance
    }

    fn render(&self) -> Vec<Color> {
        let mut pixels = vec![Color(0.0,0.0,0.0); (self.width * self.height) as usize];
        let pixel_array = pixels.as_mut_slice();

        for j in 0..self.height {
            for i in 0..self.width {
                for _ in 0..self.samples_per_pixel {
                    pixel_array[(i + j * self.width) as usize] += self.calculate_ray(Ray {
                        origin: V3(i as f32 / self.width as f32 + rand::random::<f32>(), j as f32 / self.height as f32 + rand::random::<f32>(), 0.0),
                        direction: V3(0.0, 0.0, 1.0),
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

fn read_scene(path: &str) -> Result<Scene, failure::Error> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let val = serde_yaml::from_reader(reader)?;

    Ok(val)
}

fn main() {
    let scene = read_scene("scene.yml").unwrap();
    scene.write("./out/image.ppm").unwrap();
}
