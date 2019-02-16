use crate::vector::{V3, V3U};
use crate::color::Color;

#[derive(Debug)]
pub struct Ray {
    pub origin: V3,
    pub direction: V3U,
}

impl Ray {
    fn extend_at(&self, k: f32) -> V3 {
        self.origin + self.direction.as_v3().scale(k)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Object {
    pub center: V3,
    pub radius: f32,
    pub color: Color,
    pub emission: Color,
    pub material: Material,
}

#[derive(Serialize, Deserialize)]
pub enum Material {
    Diffuse,
}

#[derive(Clone)]
pub struct HitRecord<'a> {
    pub at: f32,
    pub point: V3,
    pub normal: V3U,
    pub object: &'a Object,
}

impl Object {
    pub fn check_hit(&self, ray: &Ray) -> Option<HitRecord> {
        let b = ray.direction.as_v3().dot(ray.origin - self.center);
        let c = (ray.origin - self.center).square_norm() - self.radius*self.radius;
        let discriminant = b * b - c;

        if discriminant > 0.0 {
            let t = -b - discriminant.sqrt();
            if t > 0.0 {
                return Some(HitRecord{
                    at: t,
                    point: ray.extend_at(t),
                    normal: V3U::from_v3(ray.extend_at(t) - self.center),
                    object: self,
                });
            }

            let t = -b + discriminant.sqrt();
            if t > 0.0 {
                return Some(HitRecord{
                    at: t,
                    point: ray.extend_at(t),
                    normal: V3U::from_v3(ray.extend_at(t) - self.center),
                    object: self,
                });
            }
        }

        None
    }

    pub fn bsdf(&self) -> f32 {
        1.0 / std::f32::consts::PI
    }

    pub fn incident_flux(&self, at: V3) -> Ray {
        loop {
            let p = V3(rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()).scale(2.0) - V3(1.0, 1.0, 1.0);
            if p.square_norm() < 1.0 {
                return Ray {
                    origin: at,
                    direction: V3U::from_v3(p),
                };
            }
        }        
    }

    pub fn flux_prob(&self, normal: V3U, ray: &Ray) -> f32 {
        normal.dot(ray.direction) / std::f32::consts::PI
    }
}
