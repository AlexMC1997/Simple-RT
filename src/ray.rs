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
            self.traj = inter.mat.pdf(&self.traj, &inter.norm);
            let tmp = self.traj.norm();
            if tmp != 0.0 {
                self.traj = &self.traj / tmp;
                self.origin = inter.pos;
                color = inter.mat.bsdf(0.0, 0.0).color_prod(&self.trace(scene, bg, depth-1));
            } else {
                color = inter.mat.bsdf(0.0, 0.0);
            }
        }

        color
    }
}