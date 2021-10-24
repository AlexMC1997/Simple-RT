use std::f32::consts::PI;

mod linear;
mod camera;
mod ray;
mod scene;

fn main() {
    let width = 256;
    let height = 256;
    println!("P3");
    println!("{} {}", width, height);
    println!("255");
    let sph = scene::Sphere{color: scene::RED.copy(), pos: linear::Vec3{x: 0.0, y: 0.0, z: -4.0}, rad: 1.0};
    let mut cam = camera::Camera {
        fov: PI as f64 * 0.5, 
        look: linear::Vec3{x: 0.0, y: 0.0, z: -1.0}, 
        pos: linear::Vec3::new(), 
        ratio: 0.0, 
        scene: scene::Scene{objects: vec![sph]},
        up: linear::Vec3{x: 0.0, y: 1.0, z: 0.0},
    };

    cam.shoot(width, height);
}
