use crate::storage::entity_allocator::Entity;

pub struct ParentComponent {
    parent: Entity,
}

impl ParentComponent {
    pub fn new(parent: Entity) -> Self {
        Self { parent }
    }

    pub fn get_parent(&self) -> Entity {
        self.parent
    }

    pub fn set_parent(&mut self, parent: Entity) {
        self.parent = parent;
    }
}