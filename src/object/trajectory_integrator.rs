use std::{cell::RefCell, rc::Rc};

use super::Object;

const TIME_STEP: f32 = 10.0;
const TIME_STEPS: i32 = 10000;

fn split_objects_by_mass_significance(objects: &Vec<Rc<RefCell<Object>>>) -> (Vec<Rc<RefCell<Object>>>, Vec<Rc<RefCell<Object>>>) {
    let mut significant_mass_objects = vec![];
    let mut negligible_mass_objects = vec![];
    for object in objects {
        if object.borrow().sphere_of_influence.is_some() {
            significant_mass_objects.push(object.clone());
        } else {
            negligible_mass_objects.push(object.clone());
        }
    }
    (significant_mass_objects, negligible_mass_objects)
}

// Simultaneously integrates all objects to calculate the trajectories for all of them from scratch
// Must be called when the simulation is started and also when significant-mass object has some force applied outside of gravity
// A very expensive operation
fn do_full_trajectory_integration(objects: &Vec<Rc<RefCell<Object>>>) {
    let (significant_mass_objects, negligible_mass_objects) = split_objects_by_mass_significance(objects);
    for i in 0..TIME_STEPS {
        
    }

}

fn do_time_step(objects: &Vec<Rc<RefCell<Object>>>) {
    for object in objects {

    }
}