use crate::{
    camera::{Camera, CameraParameters},
    sky::Sky,
    traits::Hittable,
    Color,
    ray::Ray,
    material::Bounce,
};
use rand::{
    thread_rng,
    Rng,
    prelude::*,
};
use rayon::prelude::*;
use cgmath::{ElementWise, Zero};
use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Scene<W> {
    pub samples: u32,
    pub bounces: u32,
    pub camera: CameraParameters,
    pub world: W,
    #[serde(default)]
    pub sky: Sky,
}

impl<W: 'static + Hittable + Send> Scene<W> {
    pub fn run(self, width: u32, height: u32) -> impl Iterator<Item=Vec<Color>> {
        let distr = rand::distributions::Uniform::new(0.0, 1.0);
        let (tx, rx) = crossbeam::channel::unbounded::<Vec<Color>>();
        let cam = Camera::new(self.camera.look_from, self.camera.look_at, self.camera.up, width as f64 / height as f64, self.camera.fov, self.camera.aperture, self.camera.focus_distance);

        std::thread::spawn(move || {
            for j in (0..height).rev() {
                let row: Vec<Color> = (0..width).into_par_iter().map(|i| {
                    let mut rng = thread_rng();
                     (0..self.samples).map(|_| {
                         let u = (i as f64 + rng.sample(distr)) / (width-1) as f64;
                         let v = (j as f64 + rng.sample(distr)) / (height-1) as f64;
                         let ray = cam.get_ray(&mut rng, u, v);
                         self.ray_color(&mut rng, &self.world, ray, self.bounces)
                     }).sum::<Color>() / self.samples as f64
                }).collect();
                tx.send(row).unwrap();
            }
            std::mem::drop(tx);
        });
        rx.into_iter()
    }

    fn ray_color(&self, rng: &mut ThreadRng, world: &W, ray: Ray, depth: u32) -> Color {
        if depth == 0 {
            Color::zero()
        } else {
            if let Some(h) = world.hit(&ray, 0.001, std::f64::INFINITY) {
                match h.material.scatter(&ray, &h) {
                    Bounce::Bounce(color, ray) => {
                        if depth == 1 {
                            color
                        } else {
                            let inner = self.ray_color(rng, world, ray, depth-1);
                            color.mul_element_wise(inner)
                        }
                    }
                    Bounce::Sky(ray) => self.sky.get_color(ray.dir()),
                    Bounce::Stop(col) => col,
                }
            } else {
                self.sky.get_color(ray.dir())
            }
        }
    }
}