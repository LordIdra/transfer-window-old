use nalgebra_glm::{DVec2, vec2};

use crate::{storage::entity_allocator::Entity, state::State};

use self::burn_point::BurnPoint;

pub mod burn_point;

const TIME_STEP: f64 = 0.1;
const ACCELERATION_MAGNITUDE: f64 = 10.0;

pub struct Burn {
    entity: Entity,
    parent: Entity,
    tangent_direction: DVec2,
    delta_v: DVec2, // relative to tangent direction
    current_point: BurnPoint,
    points: Vec<BurnPoint>,
}

impl Burn {
    pub fn new(state: &State, entity: Entity, parent: Entity, tangent_direction: DVec2, start_time: f64) -> Self {
        let delta_v = vec2(0.0, 0.0);
        let start_point = BurnPoint::new(state, entity, parent, start_time);
        let mut burn = Self { entity, parent, tangent_direction, delta_v, current_point: start_point.clone(), points: vec![] };
        burn.recompute_burn_points(start_point);
        burn
    }

    pub fn get_start_point(&self) -> &BurnPoint {
        self.points.first().unwrap()
    }

    pub fn get_current_point(&self) -> &BurnPoint {
        &self.current_point
    }
    
    pub fn get_end_point(&self) -> &BurnPoint {
        self.points.last().unwrap()
    }

    pub fn get_entity(&self) -> Entity {
        self.entity
    }

    pub fn get_total_dv(&self) -> f64 {
        self.delta_v.magnitude()
    }

    pub fn is_time_within_burn(&self, time: f64) -> bool {
        time > self.get_start_point().get_time() && time < self.get_end_point().get_time()
    }

    pub fn get_tangent_direction(&self) -> DVec2 {
        self.tangent_direction
    }

    pub fn get_duration(&self) -> f64 {
        self.get_total_dv() / ACCELERATION_MAGNITUDE
    }

    pub fn get_parent(&self) -> Entity {
        self.parent
    }

    pub fn get_point_at_time(&self, time: f64) -> BurnPoint {
        let time_after_start = time - self.get_start_point().get_time();
        if let Some(closest_previous_point) = self.points.get((time_after_start / TIME_STEP) as usize) {
            let delta_time = time_after_start % TIME_STEP;
            closest_previous_point.next(delta_time, self.get_absolute_acceleration())
        } else {
            self.points.last().unwrap().clone()
        }
    }

    pub fn is_finished(&self) -> bool {
        self.current_point.get_time() > self.points.last().unwrap().get_time()
    }

    pub fn get_overshot_time(&self, time: f64) -> f64 {
        time - self.points.last().unwrap().get_time()
    }

    fn get_absolute_delta_v(&self) -> DVec2 {
        vec2(
            self.delta_v.x * self.tangent_direction.x - self.delta_v.y * self.tangent_direction.y,
            self.delta_v.x * self.tangent_direction.y + self.delta_v.y * self.tangent_direction.x)
    }

    fn get_absolute_acceleration(&self) -> DVec2 {
        self.get_absolute_delta_v().normalize() * ACCELERATION_MAGNITUDE
    }

    pub fn adjust(&mut self, adjustment: DVec2) {
        self.delta_v += adjustment;
        let start_point = self.get_start_point().clone();
        self.recompute_burn_points(start_point);
    }

    fn recompute_burn_points(&mut self, start_point: BurnPoint) {
        let mut points = vec![];
        let mut point = start_point.clone();
        // We don't use a while loop because we need to compute at least 1 point (otherwise the duration of the burn is 0 which breaks stuff)
        loop {
            points.push(point.clone());
            point = point.next(TIME_STEP, self.get_absolute_acceleration());
            if point.get_time() > start_point.get_time() + self.get_duration() {
                break;
            }
        }
        self.points = points;
    }

    pub fn reset(&mut self) {
        self.current_point = self.points.first().unwrap().clone();
    }

    pub fn update(&mut self, delta_time: f64) {
        self.current_point = self.current_point.next(delta_time, self.get_absolute_acceleration());
    }
}