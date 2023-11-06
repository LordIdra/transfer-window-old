pub fn update(&mut self, delta_time: f64) {
    // Act on the first orbit, since we're consuming a trajectory
    if let Some(orbit) = self.orbits.front_mut() { 
        orbit.update(delta_time);
        if orbit.is_finished() {
            self.orbits.pop_front();
        }
    }
}

pub fn update(&mut self, delta_time: f64) {
    self.trajectory.borrow_mut().update(delta_time);
    self.sync_to_trajectory();
}