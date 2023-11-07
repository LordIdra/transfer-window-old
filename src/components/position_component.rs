use nalgebra_glm::DVec2;

pub struct PositionComponent {
    absolute_position: DVec2,
}

impl PositionComponent {
    pub fn new(absolute_position: DVec2) -> Self {
        Self { absolute_position }
    }

    pub fn get_absolute_position(&self) -> DVec2 {
        self.absolute_position
    }

    pub fn set_absolute_position(&mut self, new_absolute_position: DVec2) {
        self.absolute_position = new_absolute_position;
    }
}