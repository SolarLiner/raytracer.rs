use cgmath::{EuclideanSpace, InnerSpace, MetricSpace, Point3, Vector3};

use crate::{config, P3, V3};
use std::f64::EPSILON;
use std::ops::Neg;

#[derive(Copy, Clone, Debug)]
pub struct Positioned<T> {
    pos: P3,
    value: T,
}

impl<T: Default> Default for Positioned<T> {
    fn default() -> Self {
        Self {
            pos: Point3::origin(),
            value: T::default(),
        }
    }
}

impl<T> From<config::Positioned<T>> for Positioned<T> {
    fn from(p: config::Positioned<T>) -> Self {
        Self {
            pos: p.pos.into(),
            value: p.value,
        }
    }
}

impl<T> From<Positioned<T>> for config::Positioned<T> {
    fn from(p: Positioned<T>) -> Self {
        Self {
            pos: p.pos.into(),
            value: p.value,
        }
    }
}

impl<T> Positioned<T> {
    pub fn map<U, F: FnOnce(T) -> U>(self, mapper: F) -> Positioned<U> {
        Positioned {
            pos: self.pos,
            value: mapper(self.value),
        }
    }

    pub fn map_copy<U, F: FnOnce(&T) -> U>(&self, mapper: F) -> Positioned<U> {
        Positioned {
            pos: self.pos,
            value: mapper(&self.value),
        }
    }
}

impl Positioned<SDF> {
    pub fn sdf(&self, pos: P3) -> f64 {
        let pos = pos - self.pos;
        self.value.sdf(from_vec3(pos))
    }
    pub fn sdf_d(&self, pos: P3) -> V3 {
        let pos = pos - self.pos;
        self.value.sdf_d(from_vec3(pos))
    }
}

fn from_vec3<T>(vec: Vector3<T>) -> Point3<T> {
    Point3::new(vec.x, vec.y, vec.z)
}

#[derive(Debug)]
pub enum SDF {
    Sphere {
        radius: f64,
    },
    Plane {
        normal: V3,
    },
    Box {
        size: V3,
    },
    Rounding {
        sdf: Box<SDF>,
        amount: f64,
    },
    Union {
        left: Box<Positioned<SDF>>,
        right: Box<Positioned<SDF>>,
        smooth: f64,
    },
    Intersection {
        left: Box<Positioned<SDF>>,
        right: Box<Positioned<SDF>>,
        smooth: f64,
    },
}

impl From<config::SDF> for SDF {
    fn from(c: config::SDF) -> Self {
        match c {
            config::SDF::Sphere { radius } => Self::Sphere { radius },
            config::SDF::Plane { normal } => Self::Plane {
                normal: normal.into(),
            },
            config::SDF::Box { size } => Self::Box { size: size.into() },
            config::SDF::Rounding { sdf, amount } => Self::Rounding {
                sdf: Box::new((*sdf).into()),
                amount,
            },
            config::SDF::Union {
                left,
                right,
                smooth,
            } => Self::Union {
                left: Box::new(Positioned::from(left).map(|s| (*s).into())),
                right: Box::new(Positioned::from(right).map(|s| (*s).into())),
                smooth,
            },
            config::SDF::Intersection {
                left,
                right,
                smooth,
            } => Self::Intersection {
                left: Box::new(Positioned::from(left).map(|s| (*s).into())),
                right: Box::new(Positioned::from(right).map(|s| (*s).into())),
                smooth,
            },
        }
    }
}

impl From<SDF> for config::SDF {
    fn from(s: SDF) -> Self {
        match s {
            SDF::Sphere { radius } => config::SDF::Sphere { radius },
            SDF::Plane { normal } => config::SDF::Plane {
                normal: normal.into(),
            },
            SDF::Box { size } => config::SDF::Box { size: size.into() },
            SDF::Rounding { sdf, amount } => config::SDF::Rounding {
                sdf: Box::new((*sdf).into()),
                amount,
            },
            SDF::Union {
                left,
                right,
                smooth,
            } => config::SDF::Union {
                left: config::Positioned::from(left.map(|v| Box::new(v.into()))),
                right: config::Positioned::from(right.map(|v| Box::new(v.into()))),
                smooth,
            },
            SDF::Intersection {
                left,
                right,
                smooth,
            } => config::SDF::Intersection {
                left: config::Positioned::from(left.map(|v| Box::new(v.into()))),
                right: config::Positioned::from(right.map(|v| Box::new(v.into()))),
                smooth,
            },
        }
    }
}

impl SDF {
    pub fn sdf(&self, pos: P3) -> f64 {
        match self {
            Self::Sphere { radius } => pos.to_vec().magnitude() - radius,
            Self::Plane { normal } => pos.dot(*normal),
            Self::Box { size } => {
                let q: P3 = pos.map(f64::abs) - size;
                q.map(|v| v.max(0.0)).to_vec().magnitude() + q.y.max(q.z).max(q.x).min(0.0)
            }
            Self::Rounding { sdf, amount } => sdf.sdf(pos) - amount,
            Self::Union {
                left,
                right,
                smooth,
            } => {
                let d1 = left.sdf(pos);
                let d2 = right.sdf(pos);
                let h = (0.5 + 0.5 * (d2 - d1) / smooth).max(0.0).min(1.0);
                lerp(d2, d1, h) - smooth * h * (1.0 - h)
            }
            Self::Intersection {
                left,
                right,
                smooth,
            } => {
                let d1 = left.sdf(pos);
                let d2 = right.sdf(pos);
                let h = (0.5 - 0.5 * (d2 - d1) / smooth).max(0.0).min(1.0);
                lerp(d2, d1, h) + smooth * h * (1.0 - h)
            }
        }
    }

    pub fn sdf_d(&self, pos: P3) -> V3 {
        match self {
            Self::Sphere { .. } => pos.to_vec().normalize().neg(),
            Self::Plane { normal } => *normal,
            _ => V3::new(
                self.sdf(pos + V3::unit_x() * EPSILON) - self.sdf(pos - V3::unit_x() * EPSILON),
                self.sdf(pos + V3::unit_y() * EPSILON) - self.sdf(pos - V3::unit_y() * EPSILON),
                self.sdf(pos + V3::unit_z() * EPSILON) - self.sdf(pos - V3::unit_z() * EPSILON),
            )
            .normalize(),
        }
    }
}

fn lerp(a: f64, b: f64, x: f64) -> f64 {
    (1.0 - x) * a + x * b
}
