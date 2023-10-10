use nalgebra_glm::Vec2;

pub struct VisualOrbitPoint {
    pub absolute_position: Vec2,
    pub displacement_direction: Vec2,
}

impl VisualOrbitPoint {
    pub fn new(absolute_position: Vec2, displacement_direction: Vec2) -> Self {
        Self { absolute_position, displacement_direction }
    }
}