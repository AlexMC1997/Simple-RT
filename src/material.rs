use crate::linear;
use crate::scene;

pub const BLANK: Lambert = Lambert {color: linear::Vec3 {x: 0.0, y: 0.0, z: 0.0}};
pub const RED_TESTING: Lambert = Lambert { color: linear::Vec3 {x: 0.5, y: 0.2, z: 0.1}};
pub const GREEN_TESTING: Lambert = Lambert { color: linear::Vec3 {x: 0.1, y: 0.5, z: 0.2} };
pub const BLUE_TESTING: Lambert = Lambert { color: linear::Vec3 {x: 0.1, y: 0.1, z: 0.5} };
pub const PURPLE_TESTING: Lambert = Lambert { color: linear::Vec3 {x: 0.5, y: 0.1, z: 0.5} };
pub const WHITE_TESTING: Lambert = Lambert { color: linear::Vec3 {x: 0.5, y: 0.5, z: 0.5} };
pub const MIRROR_TESTING: Specular = Specular { color: linear::Vec3 {x: 0.97, y: 0.98, z: 0.97}, roughness: 0.0 };
pub const METAL_TESTING: Specular = Specular { color: linear::Vec3 {x: 0.97, y: 0.98, z: 0.97}, roughness: 0.3 };
pub const GOLD_TESTING: Specular = Specular { color: linear::Vec3 {x: 0.98, y: 0.7, z: 0.1}, roughness: 0.7 };
pub const GLASS_TESTING: Dielectric = Dielectric { color: linear::Vec3 {x: 0.97, y: 0.98, z: 0.97}, eta: 1.3};
pub const LIGHT_TESTING: Emitter = Emitter { color: linear::Vec3 {x: 9.0, y: 9.0, z: 9.0} };
pub const REDL_TESTING: Emitter = Emitter { color: linear::Vec3 {x: 16.0, y: 0.6, z: 1.6} };
pub const BLUEL_TESTING: Emitter = Emitter { color: linear::Vec3 {x: 1.0, y: 4.4, z: 16.0} };

pub trait Material {
    fn bsdf(&self, incident: f64, exitant: f64) -> linear::Vec3<f64>;
    fn sample(&self, incident: &linear::Vec3<f64>, norm: &linear::Vec3<f64>) -> linear::Vec3<f64>;
    fn pdf(&self, exitant: &linear::Vec3<f64>, norm: &linear::Vec3<f64>) -> f64;
}

pub struct Lambert {
    pub color: linear::Vec3<f64>,
}

pub struct Specular {
    pub color: linear::Vec3<f64>,
    pub roughness: f64,
}

pub struct Dielectric {
    pub color: linear::Vec3<f64>,
    pub eta: f64,
}

pub struct Emitter {
    pub color: linear::Vec3<f64>
}

impl Material for Lambert {
    fn bsdf(&self, incident: f64, exitant: f64) -> linear::Vec3<f64> {
        self.color.copy()
    }

    fn sample(&self, incident: &linear::Vec3<f64>, norm: &linear::Vec3<f64>) -> linear::Vec3<f64> {
        let mut vec: linear::Vec3<f64> = linear::Vec3::<f64>::rand(1.0); 
        while vec.norm() > 1.0 {
            vec = linear::Vec3::<f64>::rand(1.0); 
        }
        norm + &vec
    }

    fn pdf(&self, exitant: &linear::Vec3<f64>, norm: &linear::Vec3<f64>) -> f64 {
        1.0 / 6.28318
    }
}

impl Material for Specular {
    fn bsdf(&self, incident: f64, exitant: f64) -> linear::Vec3<f64> {
        self.color.copy()
    }
    
    fn sample(&self, incident: &linear::Vec3<f64>, norm: &linear::Vec3<f64>) -> linear::Vec3<f64> {
        let refl = (-incident).reflect(norm);
        let mut vec = linear::Vec3::new();
        if self.roughness != 0.0 {
            vec = linear::Vec3::<f64>::rand(self.roughness); 
            while vec.norm() > self.roughness {
                vec = linear::Vec3::<f64>::rand(self.roughness); 
            }
        }
        let res = refl + vec;
        if &res * norm <= 0.0 {
            return scene::BLACK.copy();
        } else {
            return res;
        }
    }

    fn pdf(&self, exitant: &linear::Vec3<f64>, norm: &linear::Vec3<f64>) -> f64 {
        0.0
    }
}

impl Material for Dielectric {
    fn bsdf(&self, incident: f64, exitant: f64) -> linear::Vec3<f64> {
        self.color.copy()
    }
    
    fn sample(&self, incident: &linear::Vec3<f64>, norm: &linear::Vec3<f64>) -> linear::Vec3<f64> {
        let cos = &-incident * norm;
        let ratio = if cos >= 0.0 { self.eta.recip() } else { self.eta };
        if ratio * cos.acos().sin() > 1.0 || Dielectric::schlick(cos, ratio) > rand::random() {
            (-incident).reflect(norm)
        } else {
            (-incident).refract(norm,  ratio)
        }
    }

    fn pdf(&self, exitant: &linear::Vec3<f64>, norm: &linear::Vec3<f64>) -> f64 {
        0.0
    }
}

impl Dielectric {
    fn schlick(cos: f64, ratio: f64) -> f64 {
        let r0 = ((1.0 - ratio) / (1.0 + ratio)).powi(2);
        r0 + (1.0 - r0)*(1.0 - cos).powi(5)
    }
}

impl Material for Emitter {
    fn bsdf(&self, incident: f64, exitant: f64) -> linear::Vec3<f64> {
        self.color.copy()
        // if incident > 0.0 {
        // } else {
        //     scene::BLACK.copy()
        // }
    }
    
    fn sample(&self, incident: &linear::Vec3<f64>, norm: &linear::Vec3<f64>) -> linear::Vec3<f64> {
        linear::Vec3::new()
    }

    fn pdf(&self, exitant: &linear::Vec3<f64>, norm: &linear::Vec3<f64>) -> f64 {
        0.0
    }
}