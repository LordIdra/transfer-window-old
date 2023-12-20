use nalgebra_glm::DVec2;

pub trait VisualSegmentPoint {
    fn get_absolute_position(&self) -> DVec2;
    fn get_velocity_perpendicular(&self) -> DVec2;
}

#[derive(Clone)]
pub struct VisualOrbitPoint {
    pub theta: f64,
    pub absolute_position: DVec2,
    pub velocity_perpendicular: DVec2,
}

impl VisualSegmentPoint for VisualOrbitPoint {
    fn get_absolute_position(&self) -> DVec2 {
        self.absolute_position
    }

    fn get_velocity_perpendicular(&self) -> DVec2 {
        self.velocity_perpendicular
    }
}

#[derive(Clone)]
pub struct VisualBurnPoint {
    pub absolute_position: DVec2,
    pub velocity_perpendicular: DVec2,
}

impl VisualSegmentPoint for VisualBurnPoint {
    fn get_absolute_position(&self) -> DVec2 {
        self.absolute_position
    }

    fn get_velocity_perpendicular(&self) -> DVec2 {
        self.velocity_perpendicular
    }
}