use std::collections::BinaryHeap;
use ordered_float::NotNan;
use rand;

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
        let r = (num::abs(color.x).clamp(0.0, 1.0) * 255.0) as u8;
        let g = (num::abs(color.y).clamp(0.0, 1.0) * 255.0) as u8;
        let b = (num::abs(color.z).clamp(0.0, 1.0) * 255.0) as u8;
        print!("{:0>3} {:0>3} {:0>3}   ", r, g, b);
    }
    pub fn shoot(&mut self, width: u32, height: u32, samples: u8) {
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
        let scale = (samples as f64).recip();
        let mut cur: linear::Vec3<f64>;

        for i in 0..height {
            cur = &left + &(&rowInc * ((i as i32) - (height / 2) as i32) as f64);
            let bal = i as f64 / height as f64;
            for j in 0..width {
                let mut fcolor = linear::Vec3::new();
                for k in 0..samples {
                    let rannum1: f64 = rand::random();
                    let rannum2: f64 = rand::random();
                    let curv = &cur + &(&(&rowInc * rannum1) + &(&colInc * rannum2));
                    let mut ray = ray::Ray{color: linear::Vec3::new(), origin: self.pos.copy(), traj: curv.normalize()};

                    fcolor = &fcolor + &ray.trace(&self.scene, &(&scene::SKY + &(&scene::WHITE * bal)), 15);
                }

                Camera::write_color(&(&fcolor * scale));
                cur = &cur + &colInc;
            }
            print!("\n");
        }
    }
}

