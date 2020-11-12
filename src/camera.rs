use cgmath::{ElementWise, EuclideanSpace, InnerSpace, Vector2};

use crate::ray::Ray;
use crate::{P3, V3};
use rand::prelude::ThreadRng;
use serde::Deserialize;

#[derive(Clone, Debug)]
pub struct Camera {
    origin: P3,
    lower_left_corner: P3,
    horizontal: V3,
    vertical: V3,
    u: V3,
    v: V3,
    w: V3,
    lens_radius: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            P3::origin(),
            P3::new(0.0, 0.0, -1.0),
            V3::unit_y(),
            16.0 / 9.0,
            45.0,
            0.1,
            1.0,
        )
    }
}

impl Camera {
    pub fn new(
        look_from: P3,
        look_at: P3,
        up: V3,
        aspect_ratio: f64,
        vfov: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w: V3 = (look_from - look_at).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal: V3 = focus_dist * viewport_width * u;
        let vertical: V3 = focus_dist * viewport_height * v;

        let lens_radius = aperture / 2.0;

        Self {
            origin,
            horizontal,
            vertical,
            u,
            v,
            w,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w,
            lens_radius,
        }
    }
    pub fn get_ray(&self, rng: &mut ThreadRng, s: f64, t: f64) -> Ray {
        let rd: V3 = self.lens_radius * crate::utils::random_in_unit_disk(rng);
        let offset: V3 = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct CameraParameters {
    pub look_from: P3,
    pub look_at: P3,
    pub up: V3,
    pub fov: f64,
    pub aperture: f64,
    pub focus_distance: f64,
}