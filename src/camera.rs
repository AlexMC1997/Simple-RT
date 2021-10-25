use std::collections::BinaryHeap;
use ordered_float::NotNan;

use crate::linear;
use crate::scene;
use crate::ray;
use crate::scene::Scene;
use crate::scene::SceneObject;

type MinNonNan = std::cmp::Reverse<NotNan<f64>>;
pub struct Camera<'a> {
    pub pos: linear::Vec3<f64>,
    pub look: linear::Vec3<f64>,
    pub up: linear::Vec3<f64>,
    pub ratio: f64,
    pub fov: f64,
    pub scene: scene::Scene<'a>,
}

impl<'a> Camera<'a> {
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
        let right = self.look.rotate(&self.up, self.fov / 4.0);
        let left = self.look.rotate(&self.up, -self.fov / 4.0);
        let bot = self.look.rotate(&handle, vfov / 4.0);
        let top = self.look.rotate(&handle, -vfov / 4.0);
        let colInc = &(&right - &left) / width as f64;
        let rowInc = &(&bot - &top) / height as f64;
        let mut cur: linear::Vec3<f64>;

        for i in 0..height {
            cur = &left + &(&rowInc * ((i as i32) - (height / 2) as i32) as f64);
            let bal = i as f64 / height as f64;
            for j in 0..width {
                let ray = ray::Ray{color: linear::Vec3::new(), origin: self.pos.copy(), traj: cur.normalize()};
                let mut theap: BinaryHeap<MinNonNan> = std::collections::BinaryHeap::new();
                let mut color = &(&scene::SKY * (1.0 - bal)) + &(&scene::WHITE * bal);

                for obj in &self.scene.objects {
                    let inter = obj.intersect(&ray);
                    if inter.t >= 0.0 && &ray.traj * &inter.norm <= 0.0 {
                        if !theap.peek().is_none() {
                            let prev = theap.peek().unwrap().clone();
                            theap.push(MinNonNan{0: NotNan::new(inter.t).unwrap()});
                            if prev != theap.peek().unwrap().clone() {
                                color = inter.color;
                            }
                        } else {
                            theap.push(MinNonNan{0: NotNan::new(inter.t).unwrap()});
                            color = inter.color;
                        }
                    }
                }

                Camera::write_color(&color);
                cur = &cur + &colInc;
            }
            print!("\n");
        }
    }
}

