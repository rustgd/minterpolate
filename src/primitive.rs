use mint::{Quaternion, Vector3};

/// Interpolation primitive, defines basic arithmetic needed for interpolation.
pub trait InterpolationPrimitive: Sized {
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, scalar: f32) -> Self;
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
            x : self.x + other.x,
            y : self.x + other.y,
            z : self.x + other.z,
        }
    }

    fn sub(&self, other: &Self) -> Self {
        Vector3 {
            x : self.x - other.x,
            y : self.x - other.y,
            z : self.x - other.z,
        }
    }

    fn mul(&self, other: f32) -> Self {
        Vector3 {
            x : self.x * other,
            y : self.x * other,
            z : self.x * other,
        }
    }

    fn magnitude2(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
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

    fn magnitude2(&self) -> f32 {
        (self.s * self.s) + self.v.magnitude2()
    }
}
