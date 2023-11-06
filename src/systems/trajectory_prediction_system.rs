use nalgebra_glm::DVec2;

const SIMULATION_TIME_STEP: f64 = 200.0;
const SIMULATION_TIME_STEPS: i32 = 20000;

pub fn predict(&mut self, delta_time: f64) {
    if let Some(orbit) = self.orbits.back_mut() {
        orbit.predict(delta_time);
    }
}

pub fn change_parent(&mut self, components: &Components, entity: Entity, new_parent: Entity, time: f64) {
    // Switch frames of reference
    let new_position = components.position_components.get(&entity).unwrap().get_absolute_position() - components.position_components.get(&new_parent).unwrap().get_absolute_position();
    let new_velocity = components.velocity_components.get(&entity).unwrap().get_velocity()          - components.velocity_components.get(&new_parent).unwrap().get_velocity();
    let new_orbit = Orbit::new(components, new_parent, vec3(1.0, 0.0, 0.0), new_position, new_velocity, time);
    self.orbits.push_back(new_orbit);
}

pub fn get_sphere_of_influence_squared(&self, components: &Components, mass: f64) -> Option<f64> {
    // https://en.wikipedia.org/wiki/Sphere_of_influence_(astrodynamics)
    self.orbits.front().map(|orbit| orbit.get_sphere_of_influence(components, mass).powi(2))
}

fn compute_new_parent_upper(&self, storage: &Storage, parent: &ObjectId) -> Option<ObjectId> {
    // Check if we've left the SOI of our current parent
    let Some(parent_sphere_of_influence_squared) = storage.get(parent).sphere_of_influence_squared else {
        return None;
    };
    if self.position_relative_to_parent.magnitude_squared() < parent_sphere_of_influence_squared {
        return None;
    }
    // We can unwrap since any object with an SOI must also have a parent
    Some(storage.get(parent).parent.clone().unwrap())
}

fn object_causing_highest_acceleration(&self, storage: &Storage, objects: Vec<ObjectId>) -> Option<ObjectId> {
    let highest_acceleration = 0.0;
    let mut object_causing_highest_acceleration = None;
    for object in objects {
        let acceleration = storage.get(&object).mass * GRAVITATIONAL_CONSTANT / (self.position_relative_to_parent - storage.get(&object).position_relative_to_parent).magnitude_squared();
        if acceleration > highest_acceleration {
            object_causing_highest_acceleration = Some(object);
        }
    }
    object_causing_highest_acceleration
}

fn compute_new_parent_lower(&self, storage: &Storage, parent: &ObjectId) -> Option<ObjectId> {
    // Check if we've entered the SOI of any objects with the same parent
    let mut potential_children = vec![];
    for child in &storage.get(parent).children {
        if *child == self.id { // Prevents deadlocks
            continue;
        }
        let Some(parent_sphere_of_influence_squared) = storage.get(child).sphere_of_influence_squared else {
            continue
        };
        if (self.position_relative_to_parent - storage.get(child).position_relative_to_parent).magnitude_squared() < parent_sphere_of_influence_squared {
            potential_children.push(child.clone());
        }
    }
    self.object_causing_highest_acceleration(storage, potential_children)
}


fn update_parent_for_prediction(&mut self, storage: &Storage, time: f64) {
    if let Some(parent) = &self.parent {
        if let Some(new_parent) = self.compute_new_parent_upper(storage, parent) {
            self.trajectory.borrow_mut().change_parent(storage, self, new_parent.clone(), time);
            self.parent = Some(new_parent);
        } else if let Some(new_parent) = self.compute_new_parent_lower(storage, parent) {
            self.trajectory.borrow_mut().change_parent(storage, self, new_parent.clone(), time);
            self.parent = Some(new_parent);
        }
    }
}

pub fn update_for_prediction(&mut self, storage: &Storage, delta_time: f64, time: f64) {
    self.trajectory.borrow_mut().update_for_prediction(delta_time);
    let new_position = self.trajectory.borrow().get_final_unscaled_position();
    let new_velocity = self.trajectory.borrow().get_final_velocity();
    self.update_position_and_velocity(new_position, new_velocity);
    self.update_parent_for_prediction(storage, time);
}

pub fn trajectory_prediction_system(state: &mut State, start_time: f64) {
    for _ in 0..SIMULATION_TIME_STEPS {
        for object in self.objects.values() {
            object.borrow_mut().update_for_prediction(self, SIMULATION_TIME_STEP, start_time);
        }
    }
    for object in self.objects.values_mut() {
        object.borrow_mut().reset();
    }
}