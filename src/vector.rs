use std::ops::{Sub, Add};

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct V3(pub f32, pub f32, pub f32);

impl V3 {
    pub fn zero() -> V3 {
        V3(0.0, 0.0, 0.0)
    }

    pub fn x(&self) -> f32 {
        self.0
    }

    pub fn y(&self) -> f32 {
        self.1
    }

    pub fn z(&self) -> f32 {
        self.2
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

    pub fn elem_multiply(self, other: V3) -> V3 {
        V3(
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2,
        )
    }

    pub fn nan_safe(self) -> V3 {
        V3(
            if self.0.is_nan() { 0.0 } else { self.0 },
            if self.1.is_nan() { 0.0 } else { self.1 },
            if self.2.is_nan() { 0.0 } else { self.2 },
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

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct V3U(V3);

impl V3U {
    pub fn x(&self) -> f32 {
        (self.0).0
    }

    pub fn y(&self) -> f32 {
        (self.0).1
    }

    pub fn z(&self) -> f32 {
        (self.0).2
    }

    pub fn unsafe_new(x: f32, y: f32, z: f32) -> V3U {
        V3U(V3(x,y,z))
    }

    pub fn from_v3(v: V3) -> V3U {
        V3U(v.normalize())
    }

    pub fn as_v3(self) -> V3 {
        self.0
    }

    pub fn dot(self, other: V3U) -> f32 {
        self.as_v3().dot(other.as_v3())
    }

    pub fn cross(self, other: V3U) -> V3U {
        V3U::from_v3(V3(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        ))
    }
}

#[cfg(test)]
impl Arbitrary for V3 {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        V3(
            Arbitrary::arbitrary(g),
            Arbitrary::arbitrary(g),
            Arbitrary::arbitrary(g),
        )
    }
}

#[cfg(test)]
impl Arbitrary for V3U {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        V3U::from_v3(Arbitrary::arbitrary(g))
    }
}

#[quickcheck]
fn v3_normalize_gives_unit_vector(v: V3) -> bool {
    let r = v.normalize().norm();
    0.99 <= r && r <= 1.01
}

#[quickcheck]
fn v3u_cross_gives_unit_vector(v: V3U, w: V3U) -> bool {
    let r = v.cross(w).as_v3().norm();
    0.99 <= r && r <= 1.01
}
