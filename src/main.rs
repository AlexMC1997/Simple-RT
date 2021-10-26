use std::{env::args, ops::Rem, thread};

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
    let sph = scene::Sphere{mat: &material::WHITE_TESTING, pos: linear::Vec3{x: 0.0, y: 0.0, z: -6.0}, rad: 2.0};
    let sph2 = scene::Sphere{mat: &MIRROR_TESTING, pos: linear::Vec3{x: 2.0, y: -1.0, z: -4.2}, rad: 0.8};
    let sph3 = scene::Sphere{mat: &RED_TESTING, pos: linear::Vec3{x: -1.0, y: 0.4, z: 2.0}, rad: 1.6};
    let sph4 = scene::Sphere{mat: &PURPLE_TESTING, pos: linear::Vec3{x: -2.0, y: -1.0, z: -4.2}, rad: 0.8};
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
    cam.scene.objects.push(&sph3);
    cam.scene.objects.push(&sph4);
    cam.scene.objects.push(&earth);

    cam.shoot(width, height, samples);
    cam.film
}

fn main() {
    let mut samples: u16 = 512;
    let mut width: u32 = 640;
    let mut height: u32 = 360;
    
    let args: Vec<String> = args().collect();
    if args.len() >= 2 {
        let tmp = args[1].parse();
        if tmp.is_ok() {
            samples = tmp.unwrap();
        }
    }
    if args.len() >= 4 {
        let tmp = args[2].parse();
        if tmp.is_ok() {
            width = tmp.unwrap();
        }
        let tmp = args[3].parse();
        if tmp.is_ok() {
            height = tmp.unwrap();
        }
    }
    
    let mut handles: Vec<thread::JoinHandle<Vec<linear::Vec3<f64>>>> = Vec::new();
    println!("P3");
    println!("{} {}", width, height);
    println!("255");

    let now = std::time::SystemTime::now();
    
    let sub = samples / 12;
    let rem = samples.rem(12);
    for i in 0..rem {
        handles.push(thread::spawn(move || { do_render(width, height, sub + 1) }));
    }
    if sub != 0 {
        for i in 0..(12 - rem) {
            handles.push(thread::spawn(move || { do_render(width, height, sub) }));
        }
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
