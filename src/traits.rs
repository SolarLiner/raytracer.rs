use crate::material::Material;
use crate::ray::Ray;
use cgmath::{InnerSpace, Point3, Vector3};

#[derive(Copy, Clone, Debug)]
pub struct HitRecord {
    pub point: Point3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub front_face: bool,
    pub material: Material,
}

impl HitRecord {
    pub fn from_hit(ray: &Ray, normal: Vector3<f64>, t: f64, material: Material) -> Self {
        let front_face = ray.dir().dot(normal) <= 0.0;
        Self {
            t,
            normal: if front_face { normal } else { -normal },
            point: ray.at(t),
            front_face,
            material,
        }
    }
}

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}

/*impl<T: Hittable> Hittable for [T] {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        self.iter()
            .filter_map(|obj| obj.hit(ray, tmin, tmax))
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
    }
}*/

impl<T: Hittable> Hittable for Vec<T> {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        self.iter()
            .filter_map(|obj| obj.hit(ray, tmin, tmax))
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
    }
}
