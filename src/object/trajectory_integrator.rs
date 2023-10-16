use std::sync::Arc;

use super::Object;

const TIME_STEP: f64 = 100.0;
const TIME_STEPS: i32 = 20000;

fn get_significant_mass_objects(objects: &[Arc<Object>]) -> Vec<Arc<Object>> {
    objects.iter()
        .filter(|object| object.sphere_of_influence_squared.is_some())
        .cloned()
        .collect()
}

fn do_time_step(objects: &Vec<Arc<Object>>, significant_mass_objects: &[Arc<Object>]) {
    for object in objects {
        object.update_for_trajectory_integration(significant_mass_objects, TIME_STEP);
    }
}

// Simultaneously integrates all objects to calculate the trajectories for all of them from scratch
// Must be called when the simulation is started and also when significant-mass object has some force applied outside of gravity
// A very expensive operation
pub fn do_full_trajectory_integration(objects: &Vec<Arc<Object>>) {
    let significant_mass_objects = get_significant_mass_objects(objects);
    for _ in 0..TIME_STEPS {
        do_time_step(objects, &significant_mass_objects);
    }

    for object in objects {
        object.reset_all_conics();
    }
}