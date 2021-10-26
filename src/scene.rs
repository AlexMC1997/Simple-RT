use crate::linear;
use crate::linear::Vec3;
use crate::ray;
use crate::material;

pub const WHITE: linear::Vec3<f64> = linear::Vec3{x: 1.0, y: 1.0, z: 1.0};
pub const BLACK: linear::Vec3<f64> = linear::Vec3{x: 0.0, y: 0.0, z: 0.0};
pub const RED: linear::Vec3<f64> = linear::Vec3{x: 1.0, y: 0.0, z: 0.0};
pub const PURPLE: linear::Vec3<f64> = linear::Vec3{x: 1.0, y: 0.0, z: 1.0};
pub const BLUE: linear::Vec3<f64> = linear::Vec3{x: 0.0, y: 0.0, z: 1.0};
pub const GREEN: linear::Vec3<f64> = linear::Vec3{x: 0.0, y: 1.0, z: 0.0};
pub const SKY: linear::Vec3<f64> = linear::Vec3{x: 0.2, y: 0.4, z: 0.9};
pub trait SceneObject {
    fn intersect(&self, r: &ray::Ray) -> Intersection;
}

pub struct Intersection<'a> {
    pub t: f64,
    pub pos: linear::Vec3<f64>,
    pub norm: linear::Vec3<f64>,
    pub mat: &'a dyn material::Material,
}

pub struct Sphere<'a> {
    pub pos: linear::Vec3<f64>,
    pub rad: f64,
    pub mat: &'a dyn material::Material,
}

pub struct Scene<'a> {
    pub objects: Vec<&'a dyn SceneObject>,
}

impl<'a> SceneObject for Sphere<'a> {
    fn intersect(&self, r: &ray::Ray) -> Intersection {
        let mut result = Intersection {t: -1.0, pos: linear::Vec3::new(), norm: linear::Vec3::new(), mat: &material::BLANK};
        let a = &r.traj * &r.traj;
        let b = 2.0 * &(&r.traj * &(&r.origin - &self.pos));  
        let c = &(&r.origin - &self.pos)*&(&r.origin - &self.pos) - self.rad.powi(2);
        let disc = b.powi(2) - 4.0*a*c;
        if disc >= 0.0 {
            let t1: f64 = (-b + (disc).sqrt())/(2.0*a);
            let t2: f64 = (-b - (disc).sqrt())/(2.0*a);
            if t1 >= t2 {
                result.t = t2;
                result.pos = &r.origin + &(t2 * &r.traj);
            } else if t2 > t1 {
                result.t = t1;
                result.pos = &r.origin + &(t1 * &r.traj);
            }
    
            result.norm = (&result.pos - &self.pos).normalize();
            result.mat = self.mat;
        }
        result
    }
}