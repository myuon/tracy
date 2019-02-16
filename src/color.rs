
use std::ops::{AddAssign, DivAssign};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Color(pub f32, pub f32, pub f32);

impl Color {
    pub fn red(&self) -> u8 {
        (self.0 * 255.0) as u8
    }

    pub fn green(&self) -> u8 {
        (self.1 * 255.0) as u8
    }

    pub fn blue(&self) -> u8 {
        (self.2 * 255.0) as u8
    }

    pub fn scale(self, k: f32) -> Color {
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

