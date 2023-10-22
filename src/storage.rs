use std::{collections::HashMap, cell::{RefCell, Ref}};

use crate::{object::Object, app::ObjectId};

const SIMULATION_TIME_STEP: f64 = 200.0;
const SIMULATION_TIME_STEPS: i32 = 20000;

pub struct Storage {
    objects: HashMap<ObjectId, RefCell<Object>>,
}

impl Storage {
    pub fn new() -> Self {
        let objects = HashMap::new();
        Self { objects }
    }

    pub fn add_object(&mut self, object: Object) {
        if let Some(parent) = &object.get_parent() {
            self.objects[parent].borrow_mut().add_child(object.get_id());
        }
        self.objects.insert(object.get_id(), RefCell::new(object));
    }

    pub fn get(&self, id: &ObjectId) -> Ref<Object> {
        self.objects[id].borrow()
    }

    pub fn get_object_vertices(&self) -> Vec<f32> {
        let mut vertices = vec![];
        for object in self.objects.values() {
            vertices.extend(object.borrow().get_object_vertices(self));
        }
        vertices
    }

    pub fn get_orbit_vertices(&self, zoom: f64) -> Vec<f32> {
        let mut vertices = vec![];
        for object in self.objects.values() {
            vertices.extend(object.borrow().get_orbit_vertices(self, zoom));
        }
        vertices
    }

    pub fn do_full_prediction(&mut self, start_time: f64) {
        for _ in 0..SIMULATION_TIME_STEPS {
            for object in self.objects.values() {
                object.borrow_mut().update_for_prediction(self, SIMULATION_TIME_STEP, start_time);
            }
        }
        for object in self.objects.values_mut() {
            object.borrow_mut().reset_all_conics();
        }
        
    }

    pub fn update(&mut self, delta_time: f64) {
        for object in self.objects.values_mut() {
            object.borrow_mut().update(delta_time);
        }
    }
}
