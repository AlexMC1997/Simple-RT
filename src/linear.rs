use core::ops::{Add, Mul, Neg, Sub, Div, BitXor};
use std::ops::BitOr;
use num::Float;
use rand;


pub struct Quat<T: Float> {
    r: T,
    vec: Vec3<T>,
}

impl<T: Float> Quat<T> {
    pub fn recip(&self) -> Self {
        let den = (self.r.powi(2) + (&self.vec * &self.vec)).recip();
        Quat {r: self.r * den, vec: &(-&self.vec) * den}
    }
}

impl<T: Float> Mul for &Quat<T> {
    type Output = Quat<T>;

    fn mul(self, v: Self) -> Quat<T> {
        Quat {
            r: self.r * v.r - (&self.vec * &v.vec),
            vec: &(&(&v.vec * self.r) + &(&self.vec * v.r)) + &(&self.vec ^ &v.vec)
        }
    }
}

impl<T: Float> Div for &Quat<T> {
    type Output = Quat<T>;

    fn div(self, v: Self) -> Quat<T> {
        self * &v.recip()
    }
}

pub struct Vec3<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Vec3<T> {
    pub fn dist(&self, v: &Self) -> T {
        ((self - v).norm()).sqrt()
    }

    pub fn normalize(&self) -> Self {
        self / self.norm()
    }

    pub fn norm(&self) -> T {
        (self*self).sqrt()
    }

    pub fn reflect(&self, v: &Self) -> Self {
        let proj = &(self | &v); 
        &(-self) + &(proj + proj)
    }

    pub fn copy(&self) -> Self {
        Vec3 {x: self.x, y: self.y, z: self.z}
    }

    pub fn color_prod(&self, col: &Vec3<T>) -> Vec3<T> {
        Vec3 {x: self.x * col.x, y: self.y * col.y, z: self.z * col.z}
    }

    pub fn rotate(&self, axis: &Vec3<T>, rad: T) -> Self {
        let rot = Quat {r: rad.cos(), vec: &axis.normalize() * rad.sin()};
        let tmp = Quat {r: num::traits::zero(), vec: self.copy()};
        (&(&rot * &tmp) / &rot).vec

    } 

    pub fn new() -> Self {
        Vec3 {x: num::traits::zero(), y: num::traits::zero(), z: num::traits::zero()}
    }

}

impl Vec3<f64> {
    pub fn rand() -> Self {
        let num1: f64 = (rand::random::<f64>() - 0.5) * 2.0;
        let num2: f64 = (rand::random::<f64>() - 0.5) * 2.0;
        let num3: f64 = (rand::random::<f64>() - 0.5) * 2.0;
        Vec3{x: num1, y: num2, z: num3}
    }
}

impl Vec3<f32> {
    pub fn rand() -> Self {
        let num1: f32 = (rand::random::<f32>() - 0.5_f32) * 2.0_f32;
        let num2: f32 = (rand::random::<f32>() - 0.5_f32) * 2.0_f32;
        let num3: f32 = (rand::random::<f32>() - 0.5_f32) * 2.0_f32;
        Vec3{x: num1, y: num2, z: num3}
    }
}

impl<T: Float> Add for &Vec3<T> {
    type Output = Vec3<T>;
    fn add(self, v: Self) -> Vec3<T> {
        Vec3 {x: self.x + v.x, y: self.y + v.y, z: self.z + v.z}
    }
}

impl<T: Float> Mul<T> for &Vec3<T> {
    type Output = Vec3<T>;
    fn mul(self, v: T) -> Vec3<T> {
        Vec3 {x: self.x * v, y: self.y * v, z: self.z * v}
    }
}

impl Mul<&Vec3<f32>> for f32 {
    type Output = Vec3<f32>;
    fn mul(self, v: &Vec3<f32>) -> Vec3<f32> {
        Vec3 {x: self * v.x, y: self * v.y, z: self * v.z}
    }
}

impl Mul<&Vec3<f64>> for f64 {
    type Output = Vec3<f64>;
    fn mul(self, v: &Vec3<f64>) -> Vec3<f64> {
        Vec3 {x: self * v.x, y: self * v.y, z: self * v.z}
    }
}

impl<T: Float> Mul for &Vec3<T> {
    type Output = T;
    fn mul(self, v: Self) -> T {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
}

impl<T: Float> BitOr for &Vec3<T> {
    type Output = Vec3<T>;

    fn bitor(self, v: Self) -> Vec3<T> {
        &v.normalize() * ((self * v) / v.norm())
    }
}

impl<T: Float> BitXor for &Vec3<T> {
    type Output = Vec3<T>;

    fn bitxor(self, v: Self) -> Vec3<T> {
        Vec3 { x: (self.y*v.z - v.y*self.z), y: (self.z*v.x - self.x*v.z), z: (self.x*v.y - self.y*v.x) }
    }
}

impl<T: Float> Div<T> for &Vec3<T> {
    type Output = Vec3<T>;
    fn div(self, v: T) -> Vec3<T> {
        Vec3 {x: self.x * v.recip(), y: self.y * v.recip(), z: self.z * v.recip()}
    }
}

impl<T: Float> Neg for &Vec3<T> {
    type Output = Vec3<T>;

    fn neg(self) -> Vec3<T> {
        Vec3 {x: -self.x, y: -self.y, z: -self.z}
    }
}

impl<T: Float> Sub for &Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, v: Self) -> Vec3<T> {
        self + &-v
    }
}