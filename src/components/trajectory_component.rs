use std::{collections::VecDeque, rc::Rc, cell::RefCell};

use nalgebra_glm::DVec2;

use crate::storage::entity_allocator::Entity;

use self::segment::{Segment, orbit::Orbit};

use super::Components;

pub mod segment;

pub struct TrajectoryComponent {
    segments: VecDeque<Segment>,
}

impl TrajectoryComponent {
    pub fn new(components: &Components, parent: Entity, position: DVec2, velocity: DVec2, time: f64) -> Self {
        let mut segments = VecDeque::new();
        segments.push_back(Segment::Orbit(Rc::new(RefCell::new(Orbit::new(components, parent, position, velocity, time)))));
        Self { segments }
    }

    pub fn get_segments(&self) -> &VecDeque<Segment> {
        &self.segments
    }

    pub fn get_current_segment(&self) -> Segment {
        self.segments.front().unwrap().clone()
    }

    pub fn get_final_segment(&self) -> Segment {
        self.segments.back().unwrap().clone()
    }

    pub fn add_segment(&mut self, segment: Segment) {
        self.segments.push_back(segment);
    }

    pub fn remove_segments_after(&mut self, time: f64) {
        loop {
            match self.segments.back_mut().unwrap() {
                Segment::Burn(_) => {
                    panic!("Attempt to splice a burn")
                },
                Segment::Orbit(orbit) => {
                    if orbit.borrow().get_start_time() > time {
                        self.segments.pop_back();
                    } else if orbit.borrow().is_time_within_orbit(time) {
                        orbit.borrow_mut().trim_to_end_at(time);
                    } else {
                        return;
                    }
                },
            }
        }
    }

    pub fn predict(&mut self, delta_time: f64) {
        if let Some(segment) = self.segments.back_mut() {
            segment.predict(delta_time);
        }
    }

    pub fn update(&mut self, time: f64, delta_time: f64) {
        if let Some(segment) = self.segments.front_mut() { 
            segment.update(delta_time);
            if segment.is_finished() {
                let overshot_time = segment.get_overshot_time(time);
                self.segments.pop_front();
                self.segments.front_mut().unwrap().update(overshot_time);
            }
        }
    }
}