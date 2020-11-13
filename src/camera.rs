use cgmath::{EuclideanSpace, InnerSpace};

use crate::ray::Ray;
use crate::{P3, V3, config};
use rand::prelude::ThreadRng;


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

impl From<Camera> for config::Camera {
    fn from(c: Camera) -> Self {
        // lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w,
        // focus_dist * w + lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0
        // focus_dist * w = origin - horizontal / 2.0 - vertical / 2.0 - lower_left_corner
        let fw: crate::V3 = c.origin - c.horizontal / 2.0 - c.vertical / 2.0 - c.lower_left_corner;
        let focus_dist = fw.magnitude();
        // vertical = focus_dist * (viewport_height = 2.0 * (h = tan(theta / 2)) * v;
        // vertical / focus_dist / 2.0 = tan(theta/2) * v;
        let tanv: crate::V3 = c.vertical / (focus_dist * 2.0);
        let ttheta_over_2 = tanv.magnitude();
        let fov = ttheta_over_2.atan().to_degrees();

        Self {
            pos: c.origin.into(),
            look_at: (c.w * focus_dist).into(),
            up: c.u.into(),
            focus_distance: Some(focus_dist),
            fov,
            aperture: c.lens_radius * 2.0,
        }
    }
}

impl Camera {
    pub fn from_config(c: config::Camera, aspect_ratio: f64) -> Self {
        let look_from = c.pos.into();
        let look_at = c.look_at.into();
        Self::new(look_from, look_at, c.up.into(), aspect_ratio, c.fov, c.aperture, c.focus_distance.unwrap_or_else(|| (look_at-look_from).magnitude()))
    }
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
