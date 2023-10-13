use std::{cell::RefCell, rc::Rc};

use super::Object;

const TIME_STEP: f32 = 10.0;
const TIME_STEPS: i32 = 10000;

fn split_objects_by_mass_significance(objects: &Vec<Rc<RefCell<Object>>>) -> (Vec<Rc<RefCell<Object>>>, Vec<Rc<RefCell<Object>>>) {
    let mut significant_mass_objects = vec![];
    let mut negligible_mass_objects = vec![];
    for object in objects {
        if object.borrow().sphere_of_influence_squared.is_some() {
            significant_mass_objects.push(object.clone());
        } else {
            negligible_mass_objects.push(object.clone());
        }
    }
    (significant_mass_objects, negligible_mass_objects)
}

fn do_time_step(significant_mass_objects: &Vec<Rc<RefCell<Object>>>, negligible_mass_objects: &Vec<Rc<RefCell<Object>>>) {
    for object in significant_mass_objects {
        object.borrow_mut().update_for_trajectory_integration(object.clone(), &significant_mass_objects, TIME_STEP);
    }
    for object in negligible_mass_objects {
        object.borrow_mut().update_for_trajectory_integration(object.clone(), &significant_mass_objects, TIME_STEP);
    }
}

// Simultaneously integrates all objects to calculate the trajectories for all of them from scratch
// Must be called when the simulation is started and also when significant-mass object has some force applied outside of gravity
// A very expensive operation
fn do_full_trajectory_integration(objects: &Vec<Rc<RefCell<Object>>>) {
    let (significant_mass_objects, negligible_mass_objects) = split_objects_by_mass_significance(objects);
    for _ in 0..TIME_STEPS {
        do_time_step(&significant_mass_objects, &negligible_mass_objects);
    }

    for object in objects {
        object.borrow_mut().reset_all_conics();
    }
}