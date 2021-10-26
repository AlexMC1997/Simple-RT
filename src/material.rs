use crate::linear;
use crate::scene;

pub const BLANK: Lambert = Lambert {color: linear::Vec3 {x: 0.0, y: 0.0, z: 0.0}};
pub const RED_TESTING: Lambert = Lambert { color: linear::Vec3 {x: 0.5, y: 0.2, z: 0.1}};
pub const GREEN_TESTING: Lambert = Lambert { color: linear::Vec3 {x: 0.1, y: 0.5, z: 0.2} };
pub const PURPLE_TESTING: Lambert = Lambert { color: linear::Vec3 {x: 0.5, y: 0.1, z: 0.5} };
pub const WHITE_TESTING: Lambert = Lambert { color: linear::Vec3 {x: 0.5, y: 0.5, z: 0.5} };
pub const MIRROR_TESTING: Specular = Specular { color: linear::Vec3 {x: 0.96, y: 0.98, z: 0.96} };

pub trait Material {
    fn bsdf(&self, incident: f64, outgoing: f64) -> linear::Vec3<f64>;
    fn pdf(&self, incident: &linear::Vec3<f64>, norm: &linear::Vec3<f64>) -> linear::Vec3<f64>;
}

pub struct Lambert {
    pub color: linear::Vec3<f64>,
}

pub struct Specular {
    pub color: linear::Vec3<f64>,
}

impl Material for Lambert {
    fn bsdf(&self, incident: f64, outgoing: f64) -> linear::Vec3<f64> {
        self.color.copy()
    }

    fn pdf(&self, incident: &linear::Vec3<f64>, norm: &linear::Vec3<f64>) -> linear::Vec3<f64> {
        let mut vec: linear::Vec3<f64>; 
        loop {
            vec = linear::Vec3::<f64>::rand();
            if vec.norm() <= 1.0 {
                break;
            }
        }
        norm + &vec
    }
}

impl Material for Specular {
    fn bsdf(&self, incident: f64, outgoing: f64) -> linear::Vec3<f64> {
        self.color.copy()
    }
    
    fn pdf(&self, incident: &linear::Vec3<f64>, norm: &linear::Vec3<f64>) -> linear::Vec3<f64> {
        (-incident).reflect(norm)
    }
}