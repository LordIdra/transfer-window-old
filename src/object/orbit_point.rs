use eframe::epaint::Vec2;

pub struct OrbitPoint {
    pub absolute_position: Vec2,
    pub displacement_direction: Vec2,
}

impl OrbitPoint {
    pub fn new(absolute_position: Vec2, displacement_direction: Vec2) -> Self {
        Self { absolute_position, displacement_direction }
    }
}