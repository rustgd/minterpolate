use mint::{Quaternion, Vector3};

/// Interpolation primitive, defines basic arithmetic needed for interpolation.
pub trait InterpolationPrimitive: Sized {
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, scalar: f32) -> Self;
    fn dot(&self, other: &Self) -> f32;
    fn magnitude2(&self) -> f32;
    fn magnitude(&self) -> f32 {
        self.magnitude2().sqrt()
    }
    fn normalize(&self) -> Self {
        self.mul(1. / self.magnitude())
    }
}

impl InterpolationPrimitive for Vector3<f32> {
    fn add(&self, other: &Self) -> Self {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn sub(&self, other: &Self) -> Self {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn mul(&self, other: f32) -> Self {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }

    fn dot(&self, other: &Self) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    fn magnitude2(&self) -> f32 {
        self.dot(self)
    }
}

impl InterpolationPrimitive for Quaternion<f32> {
    fn add(&self, other: &Self) -> Self {
        Quaternion {
            s: self.s + other.s,
            v: self.v.add(&other.v),
        }
    }

    fn sub(&self, other: &Self) -> Self {
        Quaternion {
            s: self.s - other.s,
            v: self.v.sub(&other.v),
        }
    }

    fn mul(&self, other: f32) -> Self {
        Quaternion {
            s: self.s * other,
            v: self.v.mul(other),
        }
    }

    fn dot(&self, other: &Self) -> f32 {
        (self.s * other.s) + self.v.dot(&other.v)
    }

    fn magnitude2(&self) -> f32 {
        self.dot(self)
    }
}

impl InterpolationPrimitive for [f32; 4] {
    fn add(&self, other: &Self) -> Self {
        [
            self[0] + other[0],
            self[1] + other[1],
            self[2] + other[2],
            self[3] + other[3],
        ]
    }

    fn sub(&self, other: &Self) -> Self {
        [
            self[0] - other[0],
            self[1] - other[1],
            self[2] - other[2],
            self[3] - other[3],
        ]
    }

    fn mul(&self, other: f32) -> Self {
        [
            self[0] * other,
            self[1] * other,
            self[2] * other,
            self[3] * other,
        ]
    }

    fn dot(&self, other: &Self) -> f32 {
        (self[0] * other[0]) + (self[1] * other[1]) + (self[2] * other[2]) + (self[3] * other[3])
    }

    fn magnitude2(&self) -> f32 {
        self.dot(self)
    }
}

impl InterpolationPrimitive for [f32; 3] {
    fn add(&self, other: &Self) -> Self {
        [self[0] + other[0], self[1] + other[1], self[2] + other[2]]
    }

    fn sub(&self, other: &Self) -> Self {
        [self[0] - other[0], self[1] - other[1], self[2] - other[2]]
    }

    fn mul(&self, other: f32) -> Self {
        [self[0] * other, self[1] * other, self[2] * other]
    }

    fn dot(&self, other: &Self) -> f32 {
        (self[0] * other[0]) + (self[1] * other[1]) + (self[2] * other[2])
    }

    fn magnitude2(&self) -> f32 {
        self.dot(self)
    }
}
