use std::f64::consts::PI;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

use num::{Float, PrimInt, Signed};

// Vec1 ----------------------------------------------------------

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct Vec1<T>(pub T) where T: Clone;

impl<T> Add for Vec1<T> where T: Add<Output=T>, T: Clone {
    type Output = Vec1<T>;
    fn add(self, other: Self) -> Self::Output {
        Vec1(self.0 + other.0)
    }
}

impl<T> Sub for Vec1<T> where T: Sub<Output=T>, T: Clone {
    type Output = Vec1<T>;
    fn sub(self, other: Self) -> Self::Output {
        Vec1(self.0 - other.0)
    }
}

impl<T> Mul<T> for Vec1<T> where T: Mul<Output=T>, T: Clone {
    type Output = Vec1<T>;
    fn mul(self, other: T) -> Self::Output {
        Vec1(self.0 * other.clone())
    }
}

impl<T> Div<T> for Vec1<T> where T: Div<Output=T>, T: Clone {
    type Output = Vec1<T>;
    fn div(self, other: T) -> Self::Output {
        Vec1(self.0 / other.clone())
    }
}

impl<T> Vec1<T> where T: Clone {
    pub fn new(x: T) -> Self {
        Vec1(x)
    }

    pub fn x(&self) -> T {
        self.0.clone()
    }

    pub fn zero() -> Vec1<T> where T: Default {
        Default::default()
    }

    pub fn mag(&self) -> T where T: Float {
        self.sqr_mag().sqrt()
    }

    pub fn sqr_mag(&self) -> T where T: Mul<Output=T>, T: Add<Output=T> {
        self.x() * self.x()
    }

    pub fn dist(&self, other: &Vec1<T>) -> T where T: Float {
        (self.clone() - other.clone()).mag()
    }

    pub fn dist_manhattan(&self, other: &Vec1<T>) -> T where T: Signed {
        (self.x() - other.x()).abs()
    }

    pub fn dot(&self, other: &Vec1<T>) -> T where T: Mul<Output=T>, T: Add<Output=T> {
        self.x() * other.x()
    }
}

impl<T> fmt::Display for Vec1<T> where T: fmt::Debug, T: Clone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

// Vec2 ----------------------------------------------------------

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct Vec2<T>(pub (T, T)) where T: Clone;

impl<T> Add for Vec2<T> where T: Add<Output=T>, T: Clone {
    type Output = Vec2<T>;
    fn add(self, other: Self) -> Self::Output {
        Vec2((self.x() + other.0.0, self.y() + other.0.1))
    }
}

impl<T> Sub for Vec2<T> where T: Sub<Output=T>, T: Clone {
    type Output = Vec2<T>;
    fn sub(self, other: Self) -> Self::Output {
        Vec2((self.x() - other.0.0, self.y() - other.0.1))
    }
}

impl<T> Mul<T> for Vec2<T> where T: Mul<Output=T>, T: Clone {
    type Output = Vec2<T>;
    fn mul(self, other: T) -> Self::Output {
        Vec2((self.x() * other.clone(), self.y() * other.clone()))
    }
}

impl<T> Div<T> for Vec2<T> where T: Div<Output=T>, T: Clone {
    type Output = Vec2<T>;
    fn div(self, other: T) -> Self::Output {
        Vec2((self.x() / other.clone(), self.y() / other.clone()))
    }
}

impl<T> Vec2<T> where T: Clone {
    pub fn new(x: T, y: T) -> Self {
        Vec2((x, y))
    }

    pub fn x(&self) -> T {
        self.0.0.clone()
    }

    pub fn y(&self) -> T {
        self.0.1.clone()
    }

    pub fn zero() -> Vec2<T> where T: Default {
        Default::default()
    }

    pub fn mag(&self) -> T where T: Float {
        self.sqr_mag().sqrt()
    }

    pub fn sqr_mag(&self) -> T where T: Mul<Output=T>, T: Add<Output=T> {
        self.x() * self.x() + self.y() * self.y()
    }

    pub fn dist(&self, other: &Vec2<T>) -> T where T: Float {
        (self.clone() - other.clone()).mag()
    }

    pub fn dist_manhattan(&self, other: &Vec2<T>) -> T where T: Signed {
        (self.x() - other.x()).abs() + (self.y() - other.y()).abs()
    }

    pub fn dot(&self, other: &Vec2<T>) -> T where T: Mul<Output=T>, T: Add<Output=T> {
        self.x() * other.x() + self.y() * other.y()
    }
}

impl<T> fmt::Display for Vec2<T> where T: fmt::Debug, T: Clone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

// Vec3 ----------------------------------------------------------

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct Vec3<T>(pub (T, T, T)) where T: Clone;

impl<T> Add for Vec3<T> where T: Add<Output=T>, T: Clone {
    type Output = Vec3<T>;
    fn add(self, other: Self) -> Self::Output {
        Vec3((self.x() + other.0.0, self.y() + other.0.1, self.z() + other.0.2))
    }
}

impl<T> Sub for Vec3<T> where T: Sub<Output=T>, T: Clone {
    type Output = Vec3<T>;
    fn sub(self, other: Self) -> Self::Output {
        Vec3((self.x() - other.0.0, self.y() - other.0.1, self.z() - other.0.2))
    }
}

impl<T> Mul<T> for Vec3<T> where T: Mul<Output=T>, T: Clone {
    type Output = Vec3<T>;
    fn mul(self, other: T) -> Self::Output {
        Vec3((self.x() * other.clone(), self.y() * other.clone(), self.z() * other.clone()))
    }
}

impl<T> Div<T> for Vec3<T> where T: Div<Output=T>, T: Clone {
    type Output = Vec3<T>;
    fn div(self, other: T) -> Self::Output {
        Vec3((self.x() / other.clone(), self.y() / other.clone(), self.z() / other.clone()))
    }
}

impl<T> Vec3<T> where T: Clone {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3((x, y, z))
    }

    pub fn x(&self) -> T {
        self.0.0.clone()
    }

    pub fn y(&self) -> T {
        self.0.1.clone()
    }

    pub fn z(&self) -> T {
        self.0.2.clone()
    }

    pub fn zero() -> Vec3<T> where T: Default {
        Default::default()
    }

    pub fn mag(&self) -> T where T: Float {
        self.sqr_mag().sqrt()
    }

    pub fn sqr_mag(&self) -> T where T: Mul<Output=T>, T: Add<Output=T> {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn dist(&self, other: &Vec3<T>) -> T where T: Float {
        (self.clone() - other.clone()).mag()
    }

    pub fn dist_manhattan(&self, other: &Vec3<T>) -> T where T: Signed {
        (self.x() - other.x()).abs() + (self.y() - other.y()).abs() + (self.z() - other.z()).abs()
    }

    pub fn dot(&self, other: &Vec3<T>) -> T where T: Mul<Output=T>, T: Add<Output=T> {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }
}

impl<T> fmt::Display for Vec3<T> where T: fmt::Debug, T: Clone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

// Vec4 ----------------------------------------------------------

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct Vec4<T>(pub (T, T, T, T)) where T: Clone;

impl<T> Add for Vec4<T> where T: Add<Output=T>, T: Clone {
    type Output = Vec4<T>;
    fn add(self, other: Self) -> Self::Output {
        Vec4((self.x() + other.0.0, self.y() + other.0.1, self.z() + other.0.2, self.w() + other.0.3))
    }
}

impl<T> Sub for Vec4<T> where T: Sub<Output=T>, T: Clone {
    type Output = Vec4<T>;
    fn sub(self, other: Self) -> Self::Output {
        Vec4((self.x() - other.0.0, self.y() - other.0.1, self.z() - other.0.2, self.w() - other.0.3))
    }
}

impl<T> Mul<T> for Vec4<T> where T: Mul<Output=T>, T: Clone {
    type Output = Vec4<T>;
    fn mul(self, other: T) -> Self::Output {
        Vec4((self.x() * other.clone(), self.y() * other.clone(), self.z() * other.clone(), self.w() * other.clone()))
    }
}

impl<T> Div<T> for Vec4<T> where T: Div<Output=T>, T: Clone {
    type Output = Vec4<T>;
    fn div(self, other: T) -> Self::Output {
        Vec4((self.x() / other.clone(), self.y() / other.clone(), self.z() / other.clone(), self.w() / other.clone()))
    }
}

impl<T> Vec4<T> where T: Clone {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Vec4((x, y, z, w))
    }

    pub fn x(&self) -> T {
        self.0.0.clone()
    }

    pub fn y(&self) -> T {
        self.0.1.clone()
    }

    pub fn z(&self) -> T {
        self.0.2.clone()
    }

    pub fn w(&self) -> T {
        self.0.3.clone()
    }

    pub fn zero() -> Vec4<T> where T: Default {
        Default::default()
    }

    pub fn mag(&self) -> T where T: Float {
        self.sqr_mag().sqrt()
    }

    pub fn sqr_mag(&self) -> T where T: Mul<Output=T>, T: Add<Output=T> {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z() + self.w() * self.w()
    }

    pub fn dist(&self, other: &Vec4<T>) -> T where T: Float {
        (self.clone() - other.clone()).mag()
    }

    pub fn dist_manhattan(&self, other: &Vec4<T>) -> T where T: Signed {
        (self.x() - other.x()).abs() + (self.y() - other.y()).abs() + (self.z() - other.z()).abs() + (self.w() - other.w()).abs()
    }

    pub fn dot(&self, other: &Vec4<T>) -> T where T: Mul<Output=T>, T: Add<Output=T> {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z() + self.w() * other.w()
    }
}

impl<T> fmt::Display for Vec4<T> where T: fmt::Debug, T: Clone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
