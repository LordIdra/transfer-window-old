use nalgebra_glm::DVec2;

use crate::{storage::entity_allocator::Entity, state::State};

use self::burn_point::BurnPoint;

pub mod burn_point;

const TIME_STEP: f64 = 0.1;

fn compute_burn_points(start_point: &BurnPoint, start_time: f64, duration: f64) -> Vec<BurnPoint> {
    let mut points = vec![];
    let mut point = start_point.clone();
    while point.get_time() < start_time + duration {
        points.push(point.clone());
        point = point.next(TIME_STEP);
    }
    points
}

pub struct Burn {
    entity: Entity,
    parent: Entity,
    tangent_direction: DVec2,
    tangent_dv: f64,
    normal_dv: f64,
    current_point: BurnPoint,
    points: Vec<BurnPoint>,
}

impl Burn {
    pub fn new(state: &State, entity: Entity, parent: Entity, tangent_direction: DVec2, start_time: f64) -> Self {
        let tangent_dv = 0.0;
        let normal_dv = 0.0;
        let acceleration = 2.0; // this will eventually depend on the spacecraft - ie rocket equation time :)
        let total_dv = 10000.0; // todo
        let duration = total_dv / acceleration;
        let start_point = BurnPoint::new(state, entity, parent, start_time);
        let points = compute_burn_points(&start_point, start_time, duration);
        Self { entity, parent, tangent_direction, tangent_dv, normal_dv, current_point: start_point, points }
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
        10000.0 //f64::sqrt(self.tangent_dv.powi(2) + self.normal_dv.powi(2))
    }

    pub fn is_time_within_burn(&self, time: f64) -> bool {
        time > self.get_start_point().get_time() && time < self.get_end_point().get_time()
    }

    pub fn get_duration(&self) -> f64 {
        let acceleration = 2.0; // this will eventually depend on the spacecraft - ie rocket equation time :)
        self.get_total_dv() / acceleration
    }

    pub fn get_parent(&self) -> Entity {
        self.parent
    }

    pub fn get_point_at_time(&self, time: f64) -> BurnPoint {
        let time_after_start = time - self.get_start_point().get_time();
        if let Some(closest_previous_point) = self.points.get((time_after_start / TIME_STEP) as usize) {
            let delta_time = time_after_start % TIME_STEP;
            closest_previous_point.next(delta_time)
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

    pub fn reset(&mut self) {
        self.current_point = self.points.first().unwrap().clone();
    }

    pub fn update(&mut self, delta_time: f64) {
        self.current_point = self.current_point.next(delta_time);
    }
}