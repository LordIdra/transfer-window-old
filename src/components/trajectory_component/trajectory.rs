use std::collections::VecDeque;

use nalgebra_glm::{vec3, DVec2};

use crate::{storage::Storage, state::ObjectId};

use super::{orbit::Orbit, Object};

pub struct Trajectory {
    orbits: VecDeque<Orbit>,
}

impl Trajectory {
    pub fn new(storage: &Storage, parent: Option<ObjectId>, position: DVec2, velocity: DVec2, time: f64) -> Self {
        let mut orbits = VecDeque::new();
        if let Some(parent) = parent {
            orbits.push_back(Orbit::new(storage, parent, vec3(0.0, 0.6, 1.0), position, velocity, time));
        }
        Self { orbits }
    }

    pub fn get_sphere_of_influence_squared(&self, storage: &Storage, mass: f64) -> Option<f64> {
        // https://en.wikipedia.org/wiki/Sphere_of_influence_(astrodynamics)
        self.orbits.front().map(|orbit| orbit.get_sphere_of_influence(storage, mass).powi(2))
    }

    pub fn get_orbit_vertices(&self, storage: &Storage, zoom: f64) -> Vec<f32> {
        let mut vertices = vec![];
        for orbit in &self.orbits {
            vertices.extend(orbit.get_orbit_vertices(storage, zoom));
        }
        vertices
    }

    pub fn get_current_unscaled_position(&self) -> Option<DVec2> {
        self.orbits.front().map(|orbit| orbit.get_current_unscaled_position())
    }

    pub fn get_current_velocity(&self) -> Option<DVec2> {
        self.orbits.front().map(|orbit| orbit.get_current_velocity())
    }

    pub fn get_final_unscaled_position(&self) -> Option<DVec2> {
        self.orbits.back().map(|orbit| orbit.get_end_unscaled_position())
    }

    pub fn get_final_velocity(&self) -> Option<DVec2> {
        self.orbits.back().map(|orbit| orbit.get_end_velocity())
    }

    pub fn get_current_parent(&self) -> Option<ObjectId> {
        self.orbits.front().map(|orbit| orbit.get_parent().clone())
    }

    pub fn change_parent(&mut self, storage: &Storage, object: &Object, new_parent: ObjectId, time: f64) {
        // Switch frames of reference
        let new_position = object.get_absolute_position(storage) - storage.get(&new_parent).get_absolute_position(storage);
        let new_velocity = object.get_absolute_velocity(storage, time) - storage.get(&new_parent).get_absolute_velocity(storage, time);

        let new_orbit = Orbit::new(storage, new_parent, vec3(1.0, 0.0, 0.0), new_position, new_velocity, time);
        self.orbits.push_back(new_orbit);
    }

    pub fn update(&mut self, delta_time: f64) {
        // Act on the first orbit, since we're consuming a trajectory
        if let Some(orbit) = self.orbits.front_mut() { 
            orbit.update(delta_time);
            if orbit.is_finished() {
                self.orbits.pop_front();
            }
        }
    }

    pub fn update_for_prediction(&mut self, delta_time: f64) {
        if let Some(orbit) = self.orbits.back_mut() {
            orbit.update_for_prediction(delta_time);
        }
    }
}