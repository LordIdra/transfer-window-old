use std::{cell::RefCell, rc::Rc};

use super::{Object, trajectory::Trajectory};

const TIME_STEP: f32 = 1.0;

struct TrajectoryPair {
    object: Rc<RefCell<Object>>,
    trajectory: Trajectory,
}

fn do_trajectory_integration(objects: &Vec<Rc<RefCell<Object>>>) {
    let time = 0.0;
    let mut trajectory_pairs = vec![];
    for object in objects {
        let trajectory = Trajectory::new(parent, position, velocity);
        trajectory_pairs.push(TrajectoryPair { object, trajectory });
    }
}

fn do_time_step() {

}