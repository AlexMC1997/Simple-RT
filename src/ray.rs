use crate::linear;
use crate::scene;
use crate::scene::Intersection;

pub struct Ray {
    pub origin: linear::Vec3<f64>,
    pub traj: linear::Vec3<f64>,
}

impl Ray {
    pub fn trace(&mut self, scene: &scene::Scene, bg: &linear::Vec3<f64>, depth: u8) -> linear::Vec3<f64> {
        if depth == 0 {
            return scene::BLACK.copy();
        }
        let mut color = bg.copy();
        let mut min = 1000.0;
        let mut finter: Option<Intersection> = Option::None;

        for obj in &scene.objects {
            let inter = obj.intersect(&self);
            if inter.t >= 0.001 && inter.t <= min && &self.traj * &inter.norm <= 0.0 {
                min = inter.t;
                finter.replace(inter);
            }
        }
        
        if finter.is_some() {
            let inter = finter.unwrap();
            let ncos = &-&self.traj * &inter.norm;
            let mut lcolor: linear::Vec3<f64> = linear::Vec3::new();
            for light in &scene.lights {
                let mut hit = true;
                let sray = Ray {
                    origin: inter.pos.copy(),
                    traj: light.sample(inter.pos.copy()).normalize()
                };
                let lscale = inter.mat.pdf(&sray.traj, &inter.norm);
                if lscale <= 0.001 {
                    continue;
                }
                let lcos = &sray.traj * &inter.norm;
                if lcos < 0.0 {
                    continue;
                }
                let linter = light.intersect(&sray);
                for obj in &scene.objects {
                    let sinter = obj.intersect(&sray);
                    if sinter.t >= 0.001 && sinter.t < linter.t && &sray.traj * &sinter.norm <= 0.0 {
                        hit = false;
                        break;
                    }
                }
                if hit {
                    lcolor = lcolor + (lcos * lscale * linter.mat.bsdf(1.0, 0.0));
                }
            }
            color = inter.mat.bsdf(0.0, 0.0,).color_prod(&lcolor);
            self.traj = inter.mat.sample(&self.traj, &inter.norm);
            let tmp = self.traj.norm();
            if tmp != 0.0 {
                self.traj = &self.traj / tmp;
                self.origin = inter.pos;
                color = color + inter.mat.bsdf(0.0, 0.0).color_prod(&self.trace(scene, bg, depth-1));
            } else {
                color = color + inter.mat.bsdf(0.0, 0.0);
            }
        }

        color
    }
}