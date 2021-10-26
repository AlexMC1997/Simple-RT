use std::{ops::Rem, thread};

use crate::material::{GREEN_TESTING, MIRROR_TESTING, PURPLE_TESTING, RED_TESTING};

mod linear;
mod camera;
mod ray;
mod scene;
mod material;

fn write_color(color: &linear::Vec3<f64>) {
    let r = (num::abs(color.x).clamp(0.0, 1.0) * 255.0) as u8;
    let g = (num::abs(color.y).clamp(0.0, 1.0) * 255.0) as u8;
    let b = (num::abs(color.z).clamp(0.0, 1.0) * 255.0) as u8;
    print!("{:0>3} {:0>3} {:0>3}   ", r, g, b);
}

fn develop(width: u32, height: u32, films: &Vec<Vec<linear::Vec3<f64>>>) {
    let scale = (films.len() as f64).recip();
    for i in 0..height as usize {
        for j in 0..width as usize {
            let mut color: linear::Vec3<f64> = linear::Vec3::new();
            for film in films {
                color = &color + &film[i * width as usize + j];
            }
            write_color(&(color * scale));
        }
        print!("\n");
    }
}

fn do_render(width: u32, height: u32, samples: u16) -> Vec<linear::Vec3<f64>> {
    let sph = scene::Sphere{mat: &material::WHITE_TESTING, pos: linear::Vec3{x: 0.0, y: 0.0, z: -5.0}, rad: 2.0};
    let sph2 = scene::Sphere{mat: &MIRROR_TESTING, pos: linear::Vec3{x: 2.5, y: -0.8, z: -4.0}, rad: 0.8};
    let earth = scene::Sphere{mat: &material::GREEN_TESTING, pos: linear::Vec3{x: 0.0, y: -202.0, z: -5.0}, rad: 200.0};
    let mut cam = camera::Camera {
        fov: std::f64::consts::PI as f64 * 0.5, 
        look: linear::Vec3{x: 0.0, y: 0.0, z: -1.0}, 
        pos: linear::Vec3::new(), 
        ratio: 0.0, 
        scene: scene::Scene{objects: Vec::new()},
        up: linear::Vec3{x: 0.0, y: 1.0, z: 0.0},
        film: Vec::new()
    };

    cam.scene.objects.push(&sph);
    cam.scene.objects.push(&sph2);
    cam.scene.objects.push(&earth);

    cam.shoot(width, height, samples);
    cam.film
}

fn main() {
    let samples: u16 = 512;
    let width: u32 = 1920;
    let height: u32 = 1080;
    let mut handles: Vec<thread::JoinHandle<Vec<linear::Vec3<f64>>>> = Vec::new();
    println!("P3");
    println!("{} {}", width, height);
    println!("255");

    
    let sub = samples / 12;
    let mut rem = samples.rem(12);
    let mut cnt = samples;
    let now = std::time::SystemTime::now();
    while rem != 0 {
        handles.push(thread::spawn(move || { do_render(width, height, sub + 1) }));
        rem -= 1;
        cnt -= sub + 1;
    }
    while cnt != 0 {
        handles.push(thread::spawn(move || { do_render(width, height, sub) }));
        cnt -= sub;
    }
    
    let mut films: Vec<Vec<linear::Vec3<f64>>> = Vec::new();

    for handle in handles {
        films.push(handle.join().unwrap());
    }

    develop(width, height, &films);
    match now.elapsed() {
        Ok(elapsed) => eprintln!("Seconds to render: {}", (elapsed.as_millis() as f64) / 1000.0),
        Err(elapsed) => eprintln!("Error getting time.")
    }
}
