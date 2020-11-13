use serde::{Serialize, Deserialize};

use std::path::PathBuf;

type V3 = [f64; 3];

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Camera {
    pub pos: V3,
    pub look_at: V3,
    pub up: V3,
    pub focus_distance: Option<f64>,
    #[serde(default="default_aperture")]
    pub aperture: f64,
    #[serde(default="default_fov")]
    pub fov: f64,
}

const fn default_aperture() -> f64 { 0.1 }
const fn default_fov() -> f64 { 60.0 }

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ColorInput {
    Color {
        color: V3
    },
    Texture {
        filename: PathBuf
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag="type")]
pub enum Material {
    Lambert {
        albedo: ColorInput,
    },
    Metal {
        albedo: ColorInput,
        #[serde(default="default_fuzz")]
        fuzz: f64,
    },
    Dielectric {
        albedo: ColorInput,
        ior: f64,
    }
}

const fn default_fuzz() -> f64 { 0.01 }

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag="type")]
pub enum Object {
    Sphere {
        pos: V3,
        radius: f64,
        material: Material,
    },
    Plane {
        pos: V3,
        normal: V3,
        material: Material,
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Scene<W> {
    #[serde(default="default_bounces")]
    pub bounces: u32,
    #[serde(default="default_samples")]
    pub samples: u32,
    pub camera: Camera,
    pub world: W
}

const fn default_samples() -> u32 { 100 }
const fn default_bounces() -> u32 { 100 }