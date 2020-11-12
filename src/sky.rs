use crate::{Color, V3};
use serde::Deserialize;

#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub struct Sky;

impl Sky {
    pub fn get_color(&self, dir: V3) -> Color {
        let t = 0.5 * (dir.y + 1.0);
        Color::from(V3::new(1.0, 1.0, 1.0) * (1.0 - t) + V3::new(0.5, 0.7, 1.0) * t)
    }
}
