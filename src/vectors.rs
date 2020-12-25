use std::f32::consts::PI;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Debug, Hash)]
pub struct Vector2i {
    pub x: i32,
    pub y: i32,
}

impl Add<Vector2i> for Vector2i {
    type Output = Vector2i;

    fn add(self, other: Vector2i) -> Vector2i {
        Vector2i {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<Vector2i> for Vector2i {
    type Output = Vector2i;

    fn sub(self, other: Vector2i) -> Vector2i {
        Vector2i {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<i32> for Vector2i {
    type Output = Vector2i;

    fn mul(self, other: i32) -> Vector2i {
        Vector2i {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Div<i32> for Vector2i {
    type Output = Vector2i;

    fn div(self, other: i32) -> Vector2i {
        Vector2i {
            x: self.x / other,
            y: self.y / other,
        }
    }
}


impl Vector2i {
    pub fn from_rot(degrees: f32) -> Vector2i {
        let rad = degrees * PI / 180.0f32;
        return Vector2i { x: rad.cos().round() as i32, y: rad.sin().round() as i32 };
    }

    pub fn mag(&self) -> f32 {
        return (self.sqr_mag() as f32).sqrt();
    }

    pub fn mag_i(&self) -> i32 {
        return self.mag().round() as i32;
    }

    pub fn sqr_mag(&self) -> i32 {
        return (self.x * self.x) + (self.y * self.y);
    }

    pub fn dist(&self, other: Vector2i) -> f32 {
        return (self.clone() - other).mag();
    }

    pub fn dist_i(&self, other: Vector2i) -> i32 {
        return self.dist(other) as i32;
    }

    pub fn dot(&self, other: Vector2i) -> i32 {
        return (self.x * other.x) + (self.y * other.y);
    }
}


#[derive(Clone, Debug)]
pub struct Vector2f {
    pub x: f32,
    pub y: f32,
}

impl PartialEq for Vector2f {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for Vector2f {}

impl Add<Vector2f> for Vector2f {
    type Output = Vector2f;

    fn add(self, other: Vector2f) -> Vector2f {
        Vector2f {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<Vector2f> for Vector2f {
    type Output = Vector2f;

    fn sub(self, other: Vector2f) -> Vector2f {
        Vector2f {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for Vector2f {
    type Output = Vector2f;

    fn mul(self, other: f32) -> Vector2f {
        Vector2f {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Div<f32> for Vector2f {
    type Output = Vector2f;

    fn div(self, other: f32) -> Vector2f {
        Vector2f {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

#[allow(dead_code)]
impl Vector2f {
    pub fn from_rot(degrees: f32) -> Vector2f {
        let rad = degrees * PI / 180.0f32;
        return Vector2f { x: rad.cos() as f32, y: rad.sin().round() as f32 };
    }

    pub fn round(&self) -> Vector2i {
        Vector2i {
            x: self.x.round() as i32,
            y: self.y.round() as i32,
        }
    }

    pub fn mag(&self) -> f32 {
        return (self.sqr_mag() as f32).sqrt();
    }

    pub fn sqr_mag(&self) -> f32 {
        return (self.x * self.x) + (self.y * self.y);
    }

    pub fn dist(&self, other: Vector2f) -> f32 {
        return (self.clone() - other).mag();
    }

    pub fn dot(&self, other: Vector2f) -> f32 {
        return (self.x * other.x) + (self.y * other.y);
    }
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Vector3i {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Eq for Vector3i {}

impl Add<Vector3i> for Vector3i {
    type Output = Vector3i;

    fn add(self, other: Vector3i) -> Vector3i {
        Vector3i {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Vector3i> for Vector3i {
    type Output = Vector3i;

    fn sub(self, other: Vector3i) -> Vector3i {
        Vector3i {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<i32> for Vector3i {
    type Output = Vector3i;

    fn mul(self, other: i32) -> Vector3i {
        Vector3i {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<i32> for Vector3i {
    type Output = Vector3i;

    fn div(self, other: i32) -> Vector3i {
        Vector3i {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Vector3i {
    pub fn new(x: i32, y: i32, z: i32) -> Vector3i {
        Vector3i {
            x,
            y,
            z,
        }
    }
    pub fn mag(&self) -> f32 {
        return (self.sqr_mag() as f32).sqrt();
    }

    pub fn mag_i(&self) -> i32 {
        return self.mag().round() as i32;
    }

    pub fn sqr_mag(&self) -> i32 {
        return (self.x * self.x) + (self.y * self.y) + (self.z * self.z);
    }

    pub fn dist(&self, other: Vector3i) -> f32 {
        return (self.clone() - other).mag();
    }

    pub fn dist_i(&self, other: Vector3i) -> i32 {
        return self.dist(other) as i32;
    }

    pub fn dot(&self, other: Vector3i) -> i32 {
        return (self.x * other.x) + (self.y * other.y) + (self.z * other.z);
    }
}
