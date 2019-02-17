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
    pub fn check_hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let b = ray.direction.as_v3().dot(oc);
        let c = oc.square_norm() - self.radius*self.radius;
        let discriminant = b * b - c;

        if discriminant > 0.0 {
            let t = -b - discriminant.sqrt();
            if tmin < t && t < tmax {
                let tc = ray.extend_at(t) - self.center;
                return Some(HitRecord{
                    at: t,
                    point: ray.extend_at(t),
                    normal: V3U::from_v3(if tc.dot(ray.direction.as_v3()) < 0.0 { V3::zero() - tc } else { tc }),
                    object: self,
                });
            }

            let t = -b + discriminant.sqrt();
            if tmin < t && t < tmax {
                let tc = ray.extend_at(t) - self.center;
                return Some(HitRecord{
                    at: t,
                    point: ray.extend_at(t),
                    normal: V3U::from_v3(if tc.dot(ray.direction.as_v3()) < 0.0 { V3::zero() - tc } else { tc }),
                    object: self,
                });
            }
        }

        None
    }

    pub fn bsdf(&self) -> Color {
        self.emission.scale(1.0 / std::f32::consts::PI)
    }

    pub fn incident_flux(normal: V3U) -> V3U {
        let u = if normal.x().abs() > 0.001 { V3U::unsafe_new(0.0, 1.0, 0.0) } else { V3U::unsafe_new(1.0, 0.0, 0.0) };
        let u = u.cross(normal);
        let v = normal.cross(u);

        let phi = 2.0 * std::f32::consts::PI * rand::random::<f32>();
        let cos_theta = rand::random::<f32>().sqrt();
        let vec = u.as_v3().scale(phi.cos() * cos_theta) + v.as_v3().scale(phi.sin() * cos_theta) + normal.as_v3().scale((1.0 - cos_theta * cos_theta).sqrt());

        V3U::unsafe_new(vec.x(), vec.y(), vec.z())
    }

    pub fn flux_prob(&self, normal: V3U, ray: &Ray) -> f32 {
        normal.dot(ray.direction) / std::f32::consts::PI
    }
}

#[quickcheck]
fn incident_flux_is_inside_the_hemisphere(normal: V3U) -> bool {
    let r = Object::incident_flux(normal);
    normal.dot(r) >= 0.0
}
