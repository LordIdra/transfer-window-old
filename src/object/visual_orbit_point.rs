use nalgebra_glm::DVec2;

pub struct VisualOrbitPoint {
    pub absolute_position: DVec2,
    pub displacement_direction: DVec2,
}

impl VisualOrbitPoint {
    pub fn new(absolute_position: DVec2, displacement_direction: DVec2) -> Self {
        Self { absolute_position, displacement_direction }
    }
}