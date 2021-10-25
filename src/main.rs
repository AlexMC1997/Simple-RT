use std::f32::consts::PI;

mod linear;
mod camera;
mod ray;
mod scene;

fn main() {
    let width = 640;
    let height = 360;
    println!("P3");
    println!("{} {}", width, height);
    println!("255");
    let sph = scene::Sphere{color: scene::RED.copy(), pos: linear::Vec3{x: 0.0, y: 0.0, z: -5.0}, rad: 2.0};
    let sph2 = scene::Sphere{color: scene::PURPLE.copy(), pos: linear::Vec3{x: 0.1, y: -0.4, z: -2.0}, rad: 0.5};
    let earth = scene::Sphere{color: scene::GREEN.copy(), pos: linear::Vec3{x: 0.0, y: -205.0, z: -20.0}, rad: 200.0};
    let mut cam = camera::Camera {
        fov: PI as f64 * 0.5, 
        look: linear::Vec3{x: 0.0, y: 0.0, z: -1.0}, 
        pos: linear::Vec3::new(), 
        ratio: 0.0, 
        scene: scene::Scene{objects: Vec::new()},
        up: linear::Vec3{x: 0.0, y: 1.0, z: 0.0},
    };

    cam.scene.objects.push(&sph);
    cam.scene.objects.push(&sph2);
    cam.scene.objects.push(&earth);

    cam.shoot(width, height);
}
