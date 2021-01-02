use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

use num::{Float, Signed};

// Vec1 ----------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct Vec1<T> where T: Clone {
    pub x: T,
}

impl<T> Add for Vec1<T> where T: Add<Output=T>, T: Clone {
    type Output = Vec1<T>;
    fn add(self, other: Self) -> Self::Output {
        Vec1::new(self.x + other.x)
    }
}

impl<T> Sub for Vec1<T> where T: Sub<Output=T>, T: Clone {
    type Output = Vec1<T>;
    fn sub(self, other: Self) -> Self::Output {
        Vec1::new(self.x - other.x)
    }
}

impl<T> Mul<T> for Vec1<T> where T: Mul<Output=T>, T: Clone {
    type Output = Vec1<T>;
    fn mul(self, other: T) -> Self::Output {
        Vec1::new(self.x * other.clone())
    }
}

impl<T> Div<T> for Vec1<T> where T: Div<Output=T>, T: Clone {
    type Output = Vec1<T>;
    fn div(self, other: T) -> Self::Output {
        Vec1::new(self.x / other.clone())
    }
}

impl<T> Vec1<T> where T: Clone {
    pub fn new(x: T) -> Self {
        Vec1 { x }
    }

    pub fn zero() -> Vec1<T> where T: Default {
        Default::default()
    }

    pub fn mag(&self) -> T where T: Float {
        self.sqr_mag().sqrt()
    }

    pub fn sqr_mag(&self) -> T where T: Mul<Output=T>, T: Add<Output=T> {
        self.x.clone() * self.x.clone()
    }

    pub fn dist(&self, other: &Vec1<T>) -> T where T: Float {
        (self.clone() - other.clone()).mag()
    }

    pub fn dist_manhattan(&self, other: &Vec1<T>) -> T where T: Signed {
        (self.x.clone() - other.x.clone()).abs()
    }

    pub fn dot(&self, other: &Vec1<T>) -> T where T: Mul<Output=T>, T: Add<Output=T> {
        self.x.clone() * other.x.clone()
    }
}

impl<T> fmt::Display for Vec1<T> where T: fmt::Debug, T: Clone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.x)
    }
}

// Vec2 ----------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct Vec2<T> where T: Clone {
    pub x: T,
    pub y: T,
}

impl<T> Add for Vec2<T> where T: Add<Output=T>, T: Clone {
    type Output = Vec2<T>;
    fn add(self, other: Self) -> Self::Output {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl<T> Sub for Vec2<T> where T: Sub<Output=T>, T: Clone {
    type Output = Vec2<T>;
    fn sub(self, other: Self) -> Self::Output {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl<T> Mul<T> for Vec2<T> where T: Mul<Output=T>, T: Clone {
    type Output = Vec2<T>;
    fn mul(self, other: T) -> Self::Output {
        Vec2::new(self.x * other.clone(), self.y * other.clone())
    }
}

impl<T> Div<T> for Vec2<T> where T: Div<Output=T>, T: Clone {
    type Output = Vec2<T>;
    fn div(self, other: T) -> Self::Output {
        Vec2::new(self.x / other.clone(), self.y / other.clone())
    }
}

impl<T> Vec2<T> where T: Clone {
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }

    pub fn zero() -> Vec2<T> where T: Default {
        Default::default()
    }

    pub fn mag(&self) -> T where T: Float {
        self.sqr_mag().sqrt()
    }

    pub fn sqr_mag(&self) -> T where T: Mul<Output=T>, T: Add<Output=T> {
        self.x.clone() * self.x.clone() + self.y.clone() * self.y.clone()
    }

    pub fn dist(&self, other: &Vec2<T>) -> T where T: Float {
        (self.clone() - other.clone()).mag()
    }

    pub fn dist_manhattan(&self, other: &Vec2<T>) -> T where T: Signed {
        (self.x.clone() - other.x.clone()).abs() + (self.y.clone() - other.y.clone()).abs()
    }

    pub fn dot(&self, other: &Vec2<T>) -> T where T: Mul<Output=T>, T: Add<Output=T> {
        self.x.clone() * other.x.clone() + self.y.clone() * other.y.clone()
    }
}

impl<T> fmt::Display for Vec2<T> where T: fmt::Debug, T: Clone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Vec3 ----------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct Vec3<T> where T: Clone {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Add for Vec3<T> where T: Add<Output=T>, T: Clone {
    type Output = Vec3<T>;
    fn add(self, other: Self) -> Self::Output {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<T> Sub for Vec3<T> where T: Sub<Output=T>, T: Clone {
    type Output = Vec3<T>;
    fn sub(self, other: Self) -> Self::Output {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<T> Mul<T> for Vec3<T> where T: Mul<Output=T>, T: Clone {
    type Output = Vec3<T>;
    fn mul(self, other: T) -> Self::Output {
        Vec3::new(self.x * other.clone(), self.y * other.clone(), self.z * other.clone())
    }
}

impl<T> Div<T> for Vec3<T> where T: Div<Output=T>, T: Clone {
    type Output = Vec3<T>;
    fn div(self, other: T) -> Self::Output {
        Vec3::new(self.x / other.clone(), self.y / other.clone(), self.z / other.clone())
    }
}

impl<T> Vec3<T> where T: Clone {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Vec3<T> where T: Default {
        Default::default()
    }

    pub fn mag(&self) -> T where T: Float {
        self.sqr_mag().sqrt()
    }

    pub fn sqr_mag(&self) -> T where T: Mul<Output=T>, T: Add<Output=T> {
        self.x.clone() * self.x.clone() + self.y.clone() * self.y.clone() + self.z.clone() * self.z.clone()
    }

    pub fn dist(&self, other: &Vec3<T>) -> T where T: Float {
        (self.clone() - other.clone()).mag()
    }

    pub fn dist_manhattan(&self, other: &Vec3<T>) -> T where T: Signed {
        (self.x.clone() - other.x.clone()).abs() + (self.y.clone() - other.y.clone()).abs() + (self.z.clone() - other.z.clone()).abs()
    }

    pub fn dot(&self, other: &Vec3<T>) -> T where T: Mul<Output=T>, T: Add<Output=T> {
        self.x.clone() * other.x.clone() + self.y.clone() * other.y.clone() + self.z.clone() * other.z.clone()
    }
}

impl<T> fmt::Display for Vec3<T> where T: fmt::Debug, T: Clone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Vec4 ----------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct Vec4<T> where T: Clone {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Add for Vec4<T> where T: Add<Output=T>, T: Clone {
    type Output = Vec4<T>;
    fn add(self, other: Self) -> Self::Output {
        Vec4::new(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w)
    }
}

impl<T> Sub for Vec4<T> where T: Sub<Output=T>, T: Clone {
    type Output = Vec4<T>;
    fn sub(self, other: Self) -> Self::Output {
        Vec4::new(self.x - other.x, self.y - other.y, self.z - other.z, self.w - other.w)
    }
}

impl<T> Mul<T> for Vec4<T> where T: Mul<Output=T>, T: Clone {
    type Output = Vec4<T>;
    fn mul(self, other: T) -> Self::Output {
        Vec4::new(self.x * other.clone(), self.y * other.clone(), self.z * other.clone(), self.w * other.clone())
    }
}

impl<T> Div<T> for Vec4<T> where T: Div<Output=T>, T: Clone {
    type Output = Vec4<T>;
    fn div(self, other: T) -> Self::Output {
        Vec4::new(self.x / other.clone(), self.y / other.clone(), self.z / other.clone(), self.w / other.clone())
    }
}

impl<T> Vec4<T> where T: Clone {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Vec4 { x, y, z, w }
    }

    pub fn zero() -> Vec4<T> where T: Default {
        Default::default()
    }

    pub fn mag(&self) -> T where T: Float {
        self.sqr_mag().sqrt()
    }

    pub fn sqr_mag(&self) -> T where T: Mul<Output=T>, T: Add<Output=T> {
        self.x.clone() * self.x.clone() + self.y.clone() * self.y.clone() + self.z.clone() * self.z.clone() + self.w.clone() * self.w.clone()
    }

    pub fn dist(&self, other: &Vec4<T>) -> T where T: Float {
        (self.clone() - other.clone()).mag()
    }

    pub fn dist_manhattan(&self, other: &Vec4<T>) -> T where T: Signed {
        (self.x.clone() - other.x.clone()).abs() + (self.y.clone() - other.y.clone()).abs() + (self.z.clone() - other.z.clone()).abs() + (self.w.clone() - other.w.clone()).abs()
    }

    pub fn dot(&self, other: &Vec4<T>) -> T where T: Mul<Output=T>, T: Add<Output=T> {
        self.x.clone() * other.x.clone() + self.y.clone() * other.y.clone() + self.z.clone() * other.z.clone() + self.w.clone() * other.w.clone()
    }
}

impl<T> fmt::Display for Vec4<T> where T: fmt::Debug, T: Clone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
