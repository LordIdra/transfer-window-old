use nalgebra_glm::Vec2;

pub struct PositionComponent {
    pub absolute_position: Vec2,
    pub camera_relative_position: Vec2,
}