
use std::ops::{AddAssign, DivAssign};
use crate::vector::V3;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Color(V3);

impl Color {
    pub fn black() -> Color {
        Color(V3::zero())
    }

    pub fn red(&self) -> u8 {
        (self.0.x() * 255.0) as u8
    }

    pub fn green(&self) -> u8 {
        (self.0.y() * 255.0) as u8
    }

    pub fn blue(&self) -> u8 {
        (self.0.z() * 255.0) as u8
    }

    pub fn gamma_correction(self, gamma: f32) -> Color {
        Color(V3(
            (self.0).0.powf(1.0 / gamma),
            (self.0).1.powf(1.0 / gamma),
            (self.0).2.powf(1.0 / gamma),
        ))
    }

    pub fn scale(self, k: f32) -> Color {
        Color(self.0.scale(k))
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        *self = Color(self.0 + other.0);
    }
}

impl DivAssign<f32> for Color {
    fn div_assign(&mut self, other: f32) {
        *self = Color(self.0.scale(1.0 / other));
    }
}

