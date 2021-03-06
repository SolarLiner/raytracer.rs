use std::{fs::File, time::Instant};

use cgmath::{Point3, Vector3};
use indicatif::{ProgressBar, ProgressStyle};

use crate::{objects::Object, scene::Scene};

mod camera;
mod config;
mod material;
mod objects;
mod ray;
mod scene;
mod sdf;
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
    let height = args
        .next()
        .and_then(|p| p.parse().ok())
        .unwrap_or_else(|| (width as f64 * 9.0 / 16.0) as u32);

    let file = File::open(config_file).unwrap();
    let scn = Scene::<Vec<_>>::from(
        serde_yaml::from_reader::<_, config::Scene<Vec<config::Object>>>(file).unwrap(),
    )
    .map_world::<Vec<Object>, _>(|w| w.into_iter().map(|o| o.into()).collect());
    let bar = ProgressBar::new(height as u64).with_style(
        ProgressStyle::default_bar()
            .template("[{percent:>3} %] {bar:40} [{elapsed_precise} - ETA {eta_precise}]"),
    );
    println!("P3\n{} {}\n255\n", width, height);
    let start = Instant::now();
    bar.inc(1);
    for row in scn.run(width, height) {
        bar.inc(1);
        row.into_iter().for_each(write_color);
    }
    let duration = Instant::now() - start;
    bar.finish_with_message(&format!("Duration: {:2.2} s", duration.as_secs_f32()));
}

fn write_color(col: Color) {
    let col = col
        .map(|x| x.min(1.0).max(0.0).sqrt() * 255.999)
        .map(|x| x as u8);
    println!("{} {} {}", col.x, col.y, col.z);
}
