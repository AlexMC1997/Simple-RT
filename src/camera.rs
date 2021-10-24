use crate::linear;
use crate::scene;
use crate::ray;
use crate::scene::SceneObject;
pub struct Camera {
    pub pos: linear::Vec3<f64>,
    pub look: linear::Vec3<f64>,
    pub up: linear::Vec3<f64>,
    pub ratio: f64,
    pub fov: f64,
    pub scene: scene::Scene,
}

impl Camera {
    fn write_color(color: &linear::Vec3<f64>) {
        let r = num::abs(color.x * 255.0) as u8;
        let g = num::abs(color.y * 255.0) as u8;
        let b = num::abs(color.z * 255.0) as u8;
        print!("{} {} {} ", r, g, b);
    }
    pub fn shoot(&mut self, width: u32, height: u32) {
        self.ratio = (width as f64) / (height as f64);
        // let test = linear::Vec3 {x: 1.0, y: 0.0, z: 0.0};
        let vfov = self.fov / self.ratio;
        let handle = &self.up ^ &self.look;
        let left = self.look.rotate(&self.up, self.fov / 4.0);
        let right = self.look.rotate(&self.up, -self.fov / 4.0);
        let top = self.look.rotate(&handle, vfov / 4.0);
        let bot = self.look.rotate(&handle, -vfov / 4.0);
        let colInc = &(&right - &left) / width as f64;
        let rowInc = &(&bot - &top) / height as f64;
        let mut cur: linear::Vec3<f64>;

        for i in 0..height {
            cur = &left + &(&rowInc * ((i as i32) - (height / 2) as i32) as f64);
            for j in 0..width {
                let ray = ray::Ray{color: linear::Vec3::new(), origin: self.pos.copy(), traj: cur.normalize()};
                let sph = &self.scene.objects[0];
                let inter = sph.intersect(&ray);
                if inter.t >= 0.0 {
                    Camera::write_color(&inter.color);
                } else {
                    Camera::write_color(&scene::BLUE);
                }
                cur = &cur + &colInc;
            }
            print!("\n");
        }
    }
}

