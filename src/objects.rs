use std::f64::EPSILON;

use crate::{
    config,
    material::Material,
    ray::Ray,
    sdf::SDF,
    traits::{HitRecord, Hittable},
    P3, V3,
};
use cgmath::{EuclideanSpace, InnerSpace, Matrix4, Transform, Transform3, Vector3};
use std::cell::Cell;
use std::ops::Neg;

#[derive(Debug)]
pub enum ObjectData {
    Sphere { radius: f64 },
    Plane { normal: V3 },
    SDF { sdf: SDF },
}

#[derive(Debug)]
pub struct Object {
    transform: Matrix4<f64>,
    material: Material,
    odata: ObjectData,
}

impl From<config::Object> for Object {
    fn from(o: config::Object) -> Self {
        match o {
            config::Object::Sphere {
                material,
                pos,
                radius,
            } => Self {
                transform: Matrix4::from_translation(pos.into()),
                material: material.into(),
                odata: ObjectData::Sphere { radius },
            },
            config::Object::SDF { pos, sdf, material } => Self {
                transform: Matrix4::from_translation(pos.into()),
                material: material.into(),
                odata: ObjectData::SDF { sdf: sdf.into() },
            },
            config::Object::Plane {
                material,
                pos,
                normal,
            } => Self {
                transform: Matrix4::from_translation(pos.into()),
                material: material.into(),
                odata: ObjectData::Plane {
                    normal: normal.into(),
                },
            },
        }
    }
}

impl From<Object> for config::Object {
    fn from(o: Object) -> config::Object {
        let material = o.material.into();
        let pos = o.transform.transform_point(P3::origin()).into();
        match o.odata {
            ObjectData::Sphere { radius } => config::Object::Sphere {
                radius,
                material,
                pos,
            },
            ObjectData::Plane { normal } => config::Object::Plane {
                normal: normal.into(),
                material,
                pos,
            },
            ObjectData::SDF { sdf } => config::Object::SDF {
                material,
                pos,
                sdf: sdf.into(),
            },
        }
    }
}

impl Hittable for Object {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let local_ray = ray.transformed(&self.transform);
        match &self.odata {
            ObjectData::SDF { sdf } => {
                let mut depth = tmin;
                let mut pos = local_ray.pos();
                for _ in 0..1000 {
                    let dist = sdf.sdf(pos);
                    if dist < EPSILON {
                        return Some(HitRecord::from_hit(
                            ray,
                            sdf.sdf_d(pos),
                            depth,
                            self.material,
                        ));
                    }
                    if depth > tmax {
                        return None;
                    }
                    depth += dist;
                    pos = local_ray.at(depth);
                }
                return None;
            }
            ObjectData::Sphere { radius } => {
                let oc: Vector3<_> = local_ray.pos().to_vec();
                let a = local_ray.dir().magnitude2();
                let halfb = oc.dot(local_ray.dir());
                let c = oc.magnitude2() - radius * radius;
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
                        ray.at(t).to_vec() / *radius,
                        t,
                        self.material,
                    ))
                }
            }
            ObjectData::Plane { normal } => {
                let denominator = normal.dot(local_ray.dir());
                if denominator > f64::EPSILON {
                    let t = local_ray.pos().to_vec().neg().dot(*normal) / denominator;
                    if (tmin..=tmax).contains(&t) {
                        Some(HitRecord::from_hit(ray, *normal, t, self.material))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }
}
