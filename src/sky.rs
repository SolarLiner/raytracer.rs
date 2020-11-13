use crate::{Color, V3};
use serde::Deserialize;
use cgmath::InnerSpace;

#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub struct Sky;

impl Sky {
    pub fn get_color(&self, dir: V3) -> Color {
        let theta = dir.y.atan2(dir.x);
        let phi = dir.z.acos(); // Assuming dir is constant
        if dir.dot(self.sun_dir()) > 0.998 {
            Color::new(100.0, 100.0, 100.0)
        } else {
            let t = 0.5 * (dir.y + 1.0);
            Color::from(V3::new(1.0, 1.0, 1.0) * (1.0 - t) + V3::new(0.5, 0.7, 1.0) * t)
        }
    }

    pub fn sun_dir(&self) -> V3 {
        V3::new(1.0, 1.0, 1.0).normalize()
    }

    pub fn sun_color(&self) -> Color {
        Color::new(1.0, 1.0, 1.0)
    }
}
