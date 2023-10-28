use std::{collections::HashMap, cell::{RefCell, Ref}};

use nalgebra_glm::DVec2;

use crate::{object::Object, app::ObjectId};

const SIMULATION_TIME_STEP: f64 = 200.0;
const SIMULATION_TIME_STEPS: i32 = 20000;

pub struct Storage {
    root: Option<ObjectId>,
    objects: HashMap<ObjectId, RefCell<Object>>,
}

impl Storage {
    pub fn new() -> Self {
        let root = None;
        let objects = HashMap::new();
        Self { root, objects }
    }

    pub fn add_object(&mut self, object: Object) {
        if let Some(parent) = &object.get_parent() {
            self.objects[parent].borrow_mut().add_child(object.get_id());
        }
        self.objects.insert(object.get_id(), RefCell::new(object));
    }

    pub fn set_root(&mut self, root: ObjectId) {
        self.root = Some(root)
    }

    fn get_root(&self) -> ObjectId {
        self.root.clone().expect("Object storage does not have a root")
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

    fn get_all_objects_at_layer(&self, layer: i32, objects: &Vec<ObjectId>) -> Vec<ObjectId> {
        if layer == 0 {
            return objects.clone();
        }

        let mut new_objects = vec![];
        for object in objects {
            new_objects.extend(self.get(object).get_children());
        }
        new_objects
    }

    fn breadth_first_radius_search(&self, world_position: DVec2,  max_distance_to_select_squared: f64) -> Option<ObjectId> {
        let mut layer = 0;
        let mut objects_at_layer = vec![self.get_root()];
        loop {
            let mut closest_distance_squared = f64::MAX;
            let mut closest_object = None;
            objects_at_layer = self.get_all_objects_at_layer(layer, &objects_at_layer);
            if objects_at_layer.is_empty() {
                break;
            }
            for child in &objects_at_layer {
                let distance_squared = (self.get(child).get_absolute_position(self) - world_position).magnitude_squared();
                if closest_distance_squared > distance_squared {
                    closest_distance_squared = distance_squared;
                    closest_object = Some(child.clone())
                }
            }
            if closest_object.is_some() && closest_distance_squared < max_distance_to_select_squared {
                return closest_object;
            }
            layer += 1;
        }
        None
    }

    pub fn get_selected_object(&self, world_position: DVec2, max_distance_to_select: f64) -> Option<ObjectId> {
        self.breadth_first_radius_search(world_position, max_distance_to_select.powi(2))
    }

    pub fn do_full_prediction(&mut self, start_time: f64) {
        for _ in 0..SIMULATION_TIME_STEPS {
            for object in self.objects.values() {
                object.borrow_mut().update_for_prediction(self, SIMULATION_TIME_STEP, start_time);
            }
        }
        for object in self.objects.values_mut() {
            object.borrow_mut().reset();
        }
        
    }

    pub fn update(&mut self, delta_time: f64) {
        for object in self.objects.values_mut() {
            object.borrow_mut().update(delta_time);
        }
    }
}
