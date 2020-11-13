use crate::material::Material;
use crate::ray::Ray;
use crate::traits::{HitRecord, Hittable};
use crate::{P3, config};
use cgmath::{InnerSpace, Vector3};



#[derive(Copy, Clone, Debug,)]
pub struct Sphere {
    pub(crate) center: P3,
    pub(crate) radius: f64,
    pub(crate) material: Material,
}

impl From<config::Object> for Sphere {
    fn from(o: config::Object) -> Self {
        use config::Object::*;
        match o {
            Sphere {pos, radius, material} => Self {
                center: pos.into(),
                radius,
                material: material.into(),
            },
            _ => todo!()
        }
    }
}

impl From<Sphere> for config::Object {
    fn from(s: Sphere) -> Self {
        Self::Sphere {
            pos: s.center.into(),
            radius: s.radius,
            material: s.material.into(),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let oc: Vector3<_> = ray.pos() - self.center;
        let a = ray.dir().magnitude2();
        let halfb = oc.dot(ray.dir());
        let c = oc.magnitude2() - self.radius * self.radius;
        let discriminant = halfb * halfb - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt = discriminant.sqrt();
        let r0 = (-halfb - sqrt) / a;
        let r1 = (-halfb + sqrt) / a;
        let t = if r0 < tmin && r1 < tmin {
            return None;
        } else if r0 < tmin {
            r1
        } else if r1 < tmin {
            r0
        } else {
            if r0 < r1 {
                r0
            } else {
                r1
            }
        };

        if t > tmax {
            None
        } else {
            Some(HitRecord::from_hit(
                ray,
                (ray.at(t) - self.center) / self.radius,
                t,
                self.material,
            ))
        }
    }
}
