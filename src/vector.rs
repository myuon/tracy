use std::ops::{Sub, Add};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct V3(pub f32, pub f32, pub f32);

impl V3 {
    pub fn zero() -> V3 {
        V3(0.0, 0.0, 0.0)
    }

    pub fn dot(self, other: V3) -> f32 {
        self.0 * other.0 +
        self.1 * other.1 +
        self.2 * other.2
    }

    pub fn square_norm(&self) -> f32 {
        self.dot(self.clone())
    }

    pub fn norm(&self) -> f32 {
        self.square_norm().sqrt()
    }

    pub fn scale(self, k: f32) -> V3 {
        V3(
            self.0 * k,
            self.1 * k,
            self.2 * k,
        )
    }

    pub fn normalize(&self) -> V3 {
        self.scale(1.0 / self.norm())
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

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct V3U(V3);

impl V3U {
    pub fn from_v3(v: V3) -> V3U {
        V3U(v.normalize())
    }

    pub fn as_v3(self) -> V3 {
        self.0
    }

    pub fn dot(self, other: V3U) -> f32 {
        self.as_v3().dot(other.as_v3())
    }
}

