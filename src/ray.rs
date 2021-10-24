use crate::linear;

pub struct Ray {
    pub origin: linear::Vec3<f64>,
    pub traj: linear::Vec3<f64>,
    pub color: linear::Vec3<f64>, 
}