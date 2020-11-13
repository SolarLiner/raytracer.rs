use cgmath::{ElementWise, Zero};
use rand::{prelude::*, thread_rng, Rng};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};


use crate::{camera::{Camera}, material::Bounce, ray::Ray, sky::Sky, traits::Hittable, Color, config};

#[derive(Clone, Debug)]
pub struct Scene<W> {
    pub samples: u32,
    pub bounces: u32,
    pub camera: config::Camera,
    pub world: W,
    pub sky: Sky,
}

impl<'de, H, W: Deserialize<'de> + Into<H>> From<config::Scene<W>> for Scene<H> {
    fn from(s: config::Scene<W>) -> Self {
        Self {
            samples: s.samples,
            bounces: s.bounces,
            camera: s.camera,
            world: s.world.into(),
            sky: Sky,
        }
    }
}

impl<H: Into<W>, W: Serialize> From<Scene<H>> for config::Scene<W> {
    fn from(scn: Scene<H>) -> Self {
        Self {
            bounces: scn.bounces,
            samples: scn.samples,
            world: scn.world.into(),
            camera: scn.camera.into(),
        }
    }
}

impl<W> Scene<W> {
    pub fn map_world<U, F: FnOnce(W) -> U>(self, map: F) -> Scene<U> {
        let Self {
            bounces,
            samples,
            world,
            camera,
            sky,
        } = self;
        Scene {
            bounces,
            samples,
            world: map(world),
            camera,
            sky,
        }
    }
}

impl<W: 'static + Hittable + Send> Scene<W> {
    pub fn run(self, width: u32, height: u32) -> impl Iterator<Item = Vec<Color>> {
        let distr = rand::distributions::Uniform::new(0.0, 1.0);
        let (tx, rx) = crossbeam::channel::unbounded::<Vec<Color>>();
        let cam = Camera::from_config(self.camera, width as f64 / height as f64);

        std::thread::spawn(move || {
            for j in (0..height).rev() {
                let row: Vec<Color> = (0..width)
                    .into_par_iter()
                    .map(|i| {
                        let mut rng = thread_rng();
                        (0..self.samples)
                            .map(|_| {
                                let u = (i as f64 + rng.sample(distr)) / (width - 1) as f64;
                                let v = (j as f64 + rng.sample(distr)) / (height - 1) as f64;
                                let ray = cam.get_ray(&mut rng, u, v);
                                self.ray_color(&mut rng, ray, self.bounces)
                            })
                            .sum::<Color>()
                            / self.samples as f64
                    })
                    .collect();
                tx.send(row).unwrap();
            }
            std::mem::drop(tx);
        });
        rx.into_iter()
    }

    fn ray_color(&self, rng: &mut ThreadRng, ray: Ray, depth: u32) -> Color {
        if depth == 0 {
            Color::zero()
        } else {
            if let Some(h) = self.world.hit(&ray, 0.001, std::f64::INFINITY) {
                match h.material.scatter(&ray, &h) {
                    Bounce::Bounce(color, ray) => {
                        if depth == 1 {
                            color
                        } else {
                            let inner = self.ray_color(rng, ray, depth - 1);
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
