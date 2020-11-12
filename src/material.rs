use crate::ray::Ray;
use crate::traits::HitRecord;
use crate::utils::random_vector;
use crate::Color;
use crate::V3;
use cgmath::InnerSpace;
use rand::{thread_rng, Rng};
use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Material {
    Lambert { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { transmittance: Color, ior: f64 },
}

#[derive(Copy, Clone, Debug)]
pub enum Bounce {
    Sky(Ray),
    Stop(Color),
    Bounce(Color, Ray),
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Bounce {
        match *self {
            Self::Lambert { albedo } => {
                let mut rng = thread_rng();
                let dir: V3 = hit.normal + random_vector(&mut rng);
                let dir = if near_zero(dir) { hit.normal } else { dir };

                Bounce::Bounce(albedo, Ray::new(hit.point, dir))
            }
            Self::Metal { albedo, fuzz } => {
                let mut rng = thread_rng();
                let reflected = reflect(ray.dir().normalize(), hit.normal);
                let scattered = Ray::new(hit.point, reflected + fuzz * random_vector(&mut rng));
                if scattered.dir().dot(hit.normal) > 0.0 {
                    Bounce::Bounce(albedo, scattered)
                } else {
                    Bounce::Stop(albedo)
                }
            }
            Self::Dielectric { transmittance, ior } => {
                let mut rng = thread_rng();
                let distr = rand::distributions::Uniform::new(0.0, 1.0);
                let rratio = if hit.front_face { 1.0 / ior } else { ior };
                let dir = ray.dir().normalize();
                let cos_theta = (-dir).dot(hit.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
                let cannot_refract = rratio * sin_theta > 1.0;
                let refl = reflectance(cos_theta, rratio);
                let new_dir = if cannot_refract || refl > rng.sample(distr) {
                    reflect(dir, hit.normal)
                } else {
                    refract(dir, hit.normal, rratio)
                };
                Bounce::Bounce(transmittance, Ray::new(hit.point, new_dir))
            }
        }
    }
}

fn near_zero(v: V3) -> bool {
    let v = v.map(|x| x.abs() < 1e-8);
    v.x && v.y && v.z
}

fn reflect(v: V3, n: V3) -> V3 {
    v - 2.0 * v.dot(n) * n
}

fn refract(uv: V3, n: V3, etai_over_etat: f64) -> V3 {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_out_perp: V3 = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel: V3 = -(1.0 - r_out_perp.magnitude2()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}

fn reflectance(cosine: f64, idx: f64) -> f64 {
    let r0 = (1.0 - idx) / (1.0 + idx);
    let r0 = r0*r0;
    r0 + (1.0 - r0) * (1.0-cosine).powi(5)
}