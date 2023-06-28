use rand;

use crate::linear;
use crate::scene;
use crate::ray;

pub struct Camera<'a> {
    pub pos: linear::Vec3<f64>,
    pub look: linear::Vec3<f64>,
    pub up: linear::Vec3<f64>,
    pub ratio: f64,
    pub fov: f64,
    pub scene: scene::Scene<'a>,
    pub lens_rad: f64,
    pub film: Vec<linear::Vec3<f64>>
}

impl<'a> Camera<'a> {
    pub fn look_at(&mut self, vec: &linear::Vec3<f64>) {
        let lookv = &self.look - &self.pos;
        let vecv = vec - &self.pos;
        let mut theta = (&lookv.normalize() * &vecv.normalize()).acos();
        let axis = lookv ^ vecv;
        self.up = self.up.rotate(&axis, theta).normalize();
        self.look = vec.copy();
    }

    pub fn translate(&mut self, vec: &linear::Vec3<f64>) {
        let lookv = &self.pos - &self.look;
        let vecv = &(&self.pos + &vec) - &self.look;
        let mut theta = (&lookv.normalize() * &vecv.normalize()).acos();
        let axis = lookv ^ vecv;
        self.up = self.up.rotate(&axis, theta).normalize();
        self.pos = &self.pos + &vec;
    }

    pub fn shoot(&mut self, width: u32, height: u32, samples: u16) {
        self.ratio = (width as f64) / (height as f64);
        let lookv = &self.look - &self.pos;
        let focal_len = lookv.norm();
        let vfov = self.fov / self.ratio;
        let handle = (&self.up ^ &lookv).normalize();
        let right = lookv.rotate(&self.up, self.fov / 4.0).normalize() * focal_len * (self.fov / 4.0).cos().recip();
        let left = lookv.rotate(&self.up, -self.fov / 4.0).normalize() * focal_len * (self.fov / 4.0).cos().recip();
        let bot = lookv.rotate(&handle, vfov / 4.0).normalize() * focal_len * (vfov / 4.0).cos().recip();
        let top = lookv.rotate(&handle, -vfov / 4.0).normalize() * focal_len * (vfov / 4.0).cos().recip();
        let colinc = &(&right - &left) / width as f64;
        let rowinc = &(&bot - &top) / height as f64;
        let scale = (samples as f64).recip();
        let mut cur: linear::Vec3<f64>;
        let mut last_per = 0.0;
        let mut new_per = 0.0;

        for i in 0..height {
            cur = &left + &(&rowinc * ((i as i32) - (height / 2) as i32) as f64);
            let bal = i as f64 / height as f64;
            for _j in 0..width {
                let mut fcolor = linear::Vec3::new();
                for _k in 0..samples {
                    let rannum1: f64 = rand::random();
                    let rannum2: f64 = rand::random();
                    let mut aim = &cur + &(&rowinc * rannum1 + &colinc * rannum2);
                    let mut origin = self.pos.copy();
                    if self.lens_rad > 0.0 {
                        let mut rannum1: f64 = rand::random();
                        let mut rannum2: f64 = rand::random();
                        rannum1 = (rannum1 - 0.5) * 2.0 * self.lens_rad;
                        rannum2 = (rannum2 - 0.5) * 2.0 * self.lens_rad;
                        origin = origin + (&self.up * rannum1) + (&handle * rannum2);
                        aim = &(&aim + &self.pos) - &origin;
                    }
                    let mut ray = ray::Ray{origin: origin.copy(), traj: aim.normalize()};

                    fcolor = &fcolor + &ray.trace(&self.scene, &scene::SKY_DARK, 15);
                }

                self.film.push(&fcolor * scale);
                cur = &cur + &colinc;
            }
            print!("\n");
            new_per = i as f64 / height as f64 * 100.0;
            if new_per - last_per >= 5.0 {
                eprintln!("Thread #{} is {}% done.", std::thread::current().name().unwrap(), new_per.round());
                last_per = new_per;
            }
        }
    }
}

