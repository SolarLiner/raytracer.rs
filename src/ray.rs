use crate::{P3, V3};
use cgmath::{EuclideanSpace, InnerSpace, Point3, Vector3};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    pos: P3,
    dir: V3,
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            pos: Point3::origin(),
            dir: Vector3::unit_z(),
        }
    }
}

impl Ray {
    pub fn new(pos: P3, dir: V3) -> Self {
        Self {
            pos,
            dir: dir.normalize(),
        }
    }
    pub fn at(&self, t: f64) -> P3 {
        self.pos + t * self.dir
    }
    pub fn pos(&self) -> P3 {
        self.pos
    }
    pub fn dir(&self) -> V3 {
        self.dir
    }
}
