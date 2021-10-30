use std::{env::args, f64::consts::PI, ops::Rem, thread};

mod linear;
mod camera;
mod ray;
mod scene;
mod material;


fn write_color(color: &linear::Vec3<f64>) {
    let m = color.x.max(color.y).max(color.z);
    let mut r = num::abs(color.x);
    let mut g = num::abs(color.y);
    let mut b = num::abs(color.z);
    if m > 1.0 {
        r /= m;
        g /= m;
        b /= m;
    }
    let r8 = (r * 255.0) as u8;
    let g8 = (g * 255.0) as u8;
    let b8 = (b * 255.0) as u8;
    print!("{:0>3} {:0>3} {:0>3}   ", r8, g8, b8);
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
    let sph = scene::Sphere{mat: &material::GLASS_TESTING, pos: linear::Vec3{x: 0.0, y: 1.5, z: -7.0}, rad: 1.5};
    let sph2 = scene::Sphere{mat: &material::MIRROR_TESTING, pos: linear::Vec3{x: 2.0, y: 0.8, z: -4.2}, rad: 0.8};
    let sph3 = scene::Sphere{mat: &material::RED_TESTING, pos: linear::Vec3{x: 1.0, y: 0.3, z: -2.0}, rad: 0.3};
    let sph4 = scene::Sphere{mat: &material::GOLD_TESTING, pos: linear::Vec3{x: -2.0, y: 0.8, z: -4.2}, rad: 0.8};
    let sph5 = scene::Sphere{mat: &material::GLASS_TESTING, pos: linear::Vec3{x: 0.0, y: 0.6, z: -3.4}, rad: 0.6};
    let sph6 = scene::Sphere{mat: &material::WHITE_TESTING, pos: linear::Vec3{x: -6.0, y: 2.0, z: -13.0}, rad: 2.0};
    let sph7 = scene::Sphere{mat: &material::WHITE_TESTING, pos: linear::Vec3{x: 6.0, y: 2.0, z: -13.0}, rad: 2.0};
    let sph8 = scene::Sphere{mat: &material::METAL_TESTING, pos: linear::Vec3{x: -5.0, y: 1.0, z: -3.0}, rad: 1.0};
    let sph9 = scene::Sphere{mat: &material::PURPLE_TESTING, pos: linear::Vec3{x: -1.0, y: 0.3, z: -2.0}, rad: 0.3};
    let light = scene::Sphere{mat: &material::LIGHT_TESTING, pos: linear::Vec3{x: 0.0, y: 6.0, z: -6.0}, rad: 2.0};
    let earth = scene::Sphere{mat: &material::GREEN_TESTING, pos: linear::Vec3{x: 0.0, y: -800.0, z: -5.0}, rad: 800.0};
    let red_light = scene::Sphere{mat: &material::REDL_TESTING, pos: linear::Vec3{x: -4.0, y: 1.8, z: -3.2}, rad: 0.8};
    let blue_light = scene::Sphere{mat: &material::BLUEL_TESTING, pos: linear::Vec3{x: 4.0, y: 1.8, z: -3.2}, rad: 0.8};
    let back_face = scene::Face{d: -15.0, facing: scene::FaceAxis::FaceZ, mat: &material::GREEN_TESTING, w1: -10.0, w2: 10.0, h1: 0.0, h2: 10.0};
    let right_face = scene::Face{d: -8.0, facing: scene::FaceAxis::FaceX, mat: &material::RED_TESTING, w1: 0.0, w2: 10.0, h1: -16.0, h2: 10.0};
    let left_face = scene::Face{d: 8.0, facing: scene::FaceAxis::FaceX, mat: &material::BLUE_TESTING, w1: 0.0, w2: 10.0, h1: -16.0, h2: 10.0};
    let floor_face = scene::Face{d: -0.0001, facing: scene::FaceAxis::FaceY, mat: &material::WHITE_TESTING, w1: -15.0, w2: 10.0, h1: -8.0, h2: 8.0};
    let ceiling_face = scene::Face{d: 10.0, facing: scene::FaceAxis::FaceY, mat: &material::WHITE_TESTING, w1: -15.0, w2: 10.0, h1: -8.0, h2: 8.0};
    let mut cam: camera::Camera = camera::Camera {
        fov: std::f64::consts::PI as f64 * 0.5, 
        look: linear::Vec3{x: 0.0, y: 0.0, z: -1.0}, 
        pos: linear::Vec3 {x: 0.0, y: 0.0, z: 0.0}, 
        ratio: 0.0, 
        scene: scene::Scene{objects: Vec::new()},
        up: linear::Vec3{x: 0.0, y: 1.0, z: 0.0},
        lens_rad: 0.05,
        film: Vec::new()
    };

    let ptr = linear::Vec3{x: 0.0, y: 2.0, z: -6.0};
    cam.translate(&linear::Vec3 {x: 0.0, y: 3.0, z: 6.0});
    cam.look_at(&ptr);

    cam.scene.objects.push(&light);
    cam.scene.objects.push(&sph);
    cam.scene.objects.push(&sph2);
    cam.scene.objects.push(&sph3);
    cam.scene.objects.push(&sph4);
    cam.scene.objects.push(&sph5);
    cam.scene.objects.push(&sph6);
    cam.scene.objects.push(&sph7);
    cam.scene.objects.push(&sph8);
    cam.scene.objects.push(&sph9);
    // cam.scene.objects.push(&blue_light);
    // cam.scene.objects.push(&red_light);
    // cam.scene.objects.push(&earth);
    cam.scene.objects.push(&back_face);
    cam.scene.objects.push(&right_face);
    cam.scene.objects.push(&left_face);
    cam.scene.objects.push(&floor_face);
    cam.scene.objects.push(&ceiling_face);

    // let mut sph_vec: Vec<scene::Sphere> = Vec::new();
    // let mut seed = oorandom::Rand64::new(915321); //915321 299323422

    // for _i in 0..16 {
    //     let ranr: f64 = seed.rand_float() * 0.75;
    //     let rantheta: f64 = seed.rand_float() * 2.0 * PI;
    //     let randist: f64 = (seed.rand_float() + 3.0) * 2.0;
    //     let randmat: f64 = seed.rand_float();
    //     let mat: &dyn material::Material;
    //     if randmat < 0.7 {
    //         mat = &material::GLASS_TESTING;
    //     } else if randmat < 0.9 {
    //         mat = &material::MIRROR_TESTING;
    //     } else {
    //         mat = &material::PURPLE_TESTING;
    //     }
    //     sph_vec.push(scene::Sphere{mat: mat, pos: linear::Vec3{x: randist * rantheta.cos(), y: ranr, z: randist * rantheta.sin() - 6.0}, rad: ranr});
    // }

    // for sphr in &sph_vec {
    //     cam.scene.objects.push(sphr);
    // }

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
        handles.push(thread::Builder::new().name(i.to_string()).spawn(move || { do_render(width, height, sub + 1) }).unwrap());
    }
    if sub != 0 {
        for i in 0..(12 - rem) {
            handles.push(thread::Builder::new().name((i + rem).to_string()).spawn(move || { do_render(width, height, sub) }).unwrap());
        }
    }
    
    let mut films: Vec<Vec<linear::Vec3<f64>>> = Vec::new();

    for handle in handles {
        films.push(handle.join().unwrap());
    }

    develop(width, height, &films);
    match now.elapsed() {
        Ok(elapsed) => eprintln!("Seconds to render: {}", (elapsed.as_millis() as f64) / 1000.0),
        Err(_elapsed) => eprintln!("Error getting time.")
    }
}
