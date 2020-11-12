use std::f32::MAX;
use std::fs::File;
use std::io::Read;

use cgmath::{InnerSpace, Point3, Vector3, Zero};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};
use rayon::prelude::*;

use crate::camera::{Camera, CameraParameters};
use crate::material::{Bounce, Material};
use crate::objects::Sphere;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::sky::Sky;
use crate::traits::Hittable;

mod camera;
mod material;
mod objects;
mod ray;
mod scene;
mod sky;
mod traits;
mod utils;

type P3 = Point3<f64>;
type V3 = Vector3<f64>;
type Color = V3;

fn main() {
    let mut args = std::env::args().skip(1);
    let config_file = args.next().unwrap();
    let width = args.next().and_then(|p| p.parse().ok()).unwrap_or(800);
    let height = args.next().and_then(|p| p.parse().ok()).unwrap_or_else(|| (width as f64 * 9.0 / 16.0 ) as u32);

    let file = File::open(config_file).unwrap();
    let scn: Scene<Vec<Sphere>> = serde_yaml::from_reader(file).unwrap();
    let bar = ProgressBar::new(height as u64).with_style(
        ProgressStyle::default_bar().template("{bar:40} [{elapsed_precise} - ETA {eta_precise}] {percent} %"),
    );

    println!("P3\n{} {}\n255\n", width, height);
    bar.inc(1);
    for row in scn.run(width, height) {
        bar.inc(1);
        row.into_iter().for_each(write_color);
    }
}

fn write_color(col: Color) {
    let col = col
        .map(|x| x.min(1.0).max(0.0).sqrt() * 255.999)
        .map(|x| x as u8);
    println!("{} {} {}", col.x, col.y, col.z);
}
