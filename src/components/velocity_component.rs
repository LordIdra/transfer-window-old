use nalgebra_glm::DVec2;

pub struct VelocityComponent {
    absolute_velocity: DVec2,
}

impl VelocityComponent {
    pub fn new(absolute_velocity: DVec2) -> Self {
        Self { absolute_velocity }
    }

    pub fn get_absolute_velocity(&self) -> DVec2 {
        self.absolute_velocity
    }

    pub fn set_absolute_velocity(&mut self, velocity: DVec2) {
        self.absolute_velocity = velocity;
    }
}