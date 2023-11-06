use super::entity_allocator::Entity;


struct StorageEntry<T> {
    value: T,
    generation: usize,
}

pub struct ComponentStorage<T> {
    entries: Vec<Option<StorageEntry<T>>>,
}

impl<T> ComponentStorage<T> {
    pub fn new() -> Self {
        Self { entries: vec![] }
    }

    pub fn set(&mut self, entity: Entity, value: T) {
        let index = entity.get_index();
        let generation = entity.get_generation();
        let entry = Some(StorageEntry { value, generation });
        if let Some(current_entry) = self.entries.get_mut(index) {
            *current_entry = entry;
            return;
        }
        for i in 0..(self.entries.len() - index) {
            self.entries.push(None);
        }
        self.entries.push(entry);
    }

    pub fn remove(&mut self, entity: Entity) {
        let entry = self.entries
            .get_mut(entity.get_index())
            .expect("Attempt to remove a nonexistent component");
        if let Some(entry) = entry {
            if entry.generation != entity.get_generation() {
                panic!("Attempt to remove a component with an entity that has a different generation")
            }
        }
        *entry = None;
    }

    pub fn get(&self, entity: &Entity) -> Option<&T> {
        let entry = self.entries
            .get(entity.get_index())
            .expect("Entity index out of range");
        if let Some(entry) = entry {
            if entry.generation == entity.get_generation() {
                return Some(&entry.value);
            }
        }
        None
    }

    pub fn get_mut(&self, entity: &Entity) -> Option<&mut T> {
        let entry = self.entries
            .get_mut(entity.get_index())
            .expect("Entity index out of range")
            .as_mut();
        if let Some(entry) = entry {
            if entry.generation == entity.get_generation() {
                return Some(&mut entry.value);
            }
        }
        None
    }
}
