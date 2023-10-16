use std::{sync::Arc, collections::VecDeque};

use nalgebra_glm::{vec3, DVec2};

use super::{orbit::Orbit, Object};

#[derive(Debug)]
pub struct Trajectory {
    orbits: VecDeque<Orbit>,
}

impl Trajectory {
    pub fn new(parent: Option<Arc<Object>>, position: DVec2, velocity: DVec2) -> Self {
        let mut orbits = VecDeque::new();
        if let Some(parent) = parent {
            orbits.push_back(Orbit::new(parent, vec3(0.0, 0.6, 1.0), position, velocity))
        }
        Self { orbits }
    }

    pub fn get_sphere_of_influence(&self, mass: f64) -> f64 {
        // https://en.wikipedia.org/wiki/Sphere_of_influence_(astrodynamics)
        if let Some(orbit) = self.orbits.front() {
            orbit.get_sphere_of_influence(mass)
        } else {
            0.0
        }
    }

    pub fn get_orbit_vertices(&self, zoom: f64) -> Vec<f32> {
        let mut vertices = vec![];
        for orbit in &self.orbits {
            vertices.extend(orbit.get_orbit_vertices(zoom));
        }
        vertices
    }

    pub fn get_current_unscaled_position(&self) -> Option<DVec2> {
        self.orbits.front().map(|orbit| orbit.get_unscaled_position())
    }

    pub fn get_current_velocity(&self) -> Option<DVec2> {
        self.orbits.front().map(|orbit| orbit.get_velocity())
    }

    pub fn get_final_unscaled_position(&self) -> Option<DVec2> {
        self.orbits.back().map(|orbit| orbit.get_unscaled_position())
    }

    pub fn get_final_velocity(&self) -> Option<DVec2> {
        self.orbits.back().map(|orbit| orbit.get_velocity())
    }

    pub fn get_parent(&self) -> Option<Arc<Object>> {
        self.orbits.front().map(|orbit| orbit.get_parent())
    }

    pub fn update(&mut self, delta_time: f64) {
        // Act on the first orbit, since we're consuming a trajectory
        if let Some(orbit) = self.orbits.front_mut() { 
            orbit.update(delta_time);
            if orbit.is_finished() {
                self.orbits.pop_front();
                println!("start {:?}", self.orbits.front().unwrap().debug1());
                println!("end {:?}", self.orbits.front().unwrap().debug2());
                println!("argument of periapsis {}", self.orbits.front().unwrap().debug3());
            }
        }
    }

    pub fn update_for_trajectory_integration(&mut self, object: &Object, significant_mass_objects: &[Arc<Object>], delta_time: f64) {
        // Act on the last orbit, since we're extending a trajectory
        if let Some(orbit) = self.orbits.back_mut() { 
            orbit.update(delta_time);
            if let Some(new_parent) = orbit.get_new_soi(object, significant_mass_objects) {
                if new_parent.name != orbit.get_parent().name {
                    orbit.end();
                    // Switch frames of reference
                    let new_position = object.get_absolute_position() - new_parent.get_absolute_position();
                    let new_velocity = object.get_absolute_velocity() - new_parent.get_absolute_velocity();
                    println!("{} {} {}", new_parent.name, new_position, new_velocity);
                    self.orbits.push_back(Orbit::new(new_parent, vec3(1.0, 0.0, 0.0), new_position, new_velocity));
                }
            }
        }
    }

    // Sets all orbit current points back to their starting points, and sets an endpoint for the last orbit
    pub fn reset(&mut self) {
        if let Some(orbit) = self.orbits.back_mut() {
            orbit.end()
        }
        for orbit in &mut self.orbits {
            orbit.reset();
        }
    }
}