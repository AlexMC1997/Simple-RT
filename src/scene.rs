use crate::linear;
use crate::ray;
use crate::material;

pub const WHITE: linear::Vec3<f64> = linear::Vec3{x: 1.0, y: 1.0, z: 1.0};
pub const BLACK: linear::Vec3<f64> = linear::Vec3{x: 0.0, y: 0.0, z: 0.0};
pub const RED: linear::Vec3<f64> = linear::Vec3{x: 1.0, y: 0.0, z: 0.0};
pub const PURPLE: linear::Vec3<f64> = linear::Vec3{x: 1.0, y: 0.0, z: 1.0};
pub const BLUE: linear::Vec3<f64> = linear::Vec3{x: 0.0, y: 0.0, z: 1.0};
pub const GREEN: linear::Vec3<f64> = linear::Vec3{x: 0.0, y: 1.0, z: 0.0};
pub const SKY: linear::Vec3<f64> = linear::Vec3{x: 0.2, y: 0.4, z: 0.9};
pub const SKY_DARK: linear::Vec3<f64> = linear::Vec3{x: 0.1, y: 0.1, z: 0.1};

pub enum FaceAxis {
    FaceX,
    FaceY,
    FaceZ,
}

pub trait SceneObject {
    fn intersect(&self, r: &ray::Ray) -> Intersection;
}

pub trait LightSource {
    fn sample(&self, from: linear::Vec3<f64>) -> linear::Vec3<f64>;
}

pub trait LightObject: SceneObject + LightSource {}

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

pub struct Face<'a> {
    pub h1: f64,
    pub h2: f64,
    pub w1: f64,
    pub w2: f64,
    pub d: f64,
    pub facing: FaceAxis,
    pub mat: &'a dyn material::Material,
}

pub struct Scene<'a> {
    pub objects: Vec<&'a dyn SceneObject>,
    pub lights: Vec<&'a dyn LightObject>,
}

impl<'a> SceneObject for Face<'a> {
    fn intersect(&self, r: &ray::Ray) -> Intersection {
        let mut result = Intersection {t: -1.0, pos: linear::Vec3::new(), norm: linear::Vec3::new(), mat: self.mat};
        result.norm = match self.facing {
            FaceAxis::FaceX => linear::X,
            FaceAxis::FaceY => linear::Y,
            FaceAxis::FaceZ => linear::Z,
        };
        if self.d >= 0.0 {
            result.norm = -result.norm;
        }
        let t = match self.facing {
            FaceAxis::FaceX => (self.d - r.origin.x) / r.traj.x,
            FaceAxis::FaceY => (self.d - r.origin.y) / r.traj.y,
            FaceAxis::FaceZ => (self.d - r.origin.z) / r.traj.z,
        };
        let p = &r.origin + &(t * &r.traj);
        match self.facing {
            FaceAxis::FaceX => {
                if self.w1 <= p.y && p.y <= self.w2 && self.h1 <= p.z && p.z <= self.h2 { result.t = t; }
            },
            FaceAxis::FaceY => {
                if self.w1 <= p.z && p.z <= self.w2 && self.h1 <= p.x && p.x <= self.h2 { result.t = t; }
            },
            FaceAxis::FaceZ => {
                if self.w1 <= p.x && p.x <= self.w2 && self.h1 <= p.y && p.y <= self.h2 { result.t = t; }
            },
        };
        result.pos = p;
        result
    }
}

impl<'a> SceneObject for Sphere<'a> {
    fn intersect(&self, r: &ray::Ray) -> Intersection {
        let mut result = Intersection {t: -1.0, pos: linear::Vec3::new(), norm: linear::Vec3::new(), mat: &material::BLANK};
        let pc = &r.origin - &self.pos;
        let b = &r.traj * &pc;  
        let c = &pc*&pc - self.rad.powi(2);
        let disc = b.powi(2) - c;
        if disc >= 0.0 {
            let sdisc = (disc).sqrt();
            if sdisc >= 0.0 {
                result.t = -b - sdisc;
                result.pos = &r.origin + &(result.t * &r.traj);
            } else {
                result.t = -b + sdisc;
                result.pos = &r.origin + &(result.t * &r.traj);
            }
    
            result.norm = (&result.pos - &self.pos).normalize();
            result.mat = self.mat;
        }
        result
    }
}

impl<'a> LightSource for Sphere<'a> {
    fn sample(&self, from: linear::Vec3<f64>) -> linear::Vec3<f64> {
        let mut vec = linear::Vec3::<f64>::rand(self.rad);
        while vec.norm() > self.rad {
            vec = linear::Vec3::<f64>::rand(self.rad);
        }
        &self.pos - &from + vec 
    }
}

impl<'a> Clone for Sphere<'a> {
    fn clone(&self) -> Self {
        Sphere {pos: self.pos.copy(), mat: self.mat, rad: self.rad}
    }
}

impl<'a> LightObject for Sphere<'a> {}