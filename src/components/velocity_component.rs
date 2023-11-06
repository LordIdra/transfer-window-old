use nalgebra_glm::DVec2;

pub struct VelocityComponent {
    velocity: DVec2,
}

impl VelocityComponent {
    pub fn new(velocity: DVec2) -> Self {
        Self { velocity }
    }

    pub fn get_velocity(&self) -> DVec2 {
        self.velocity
    }
}