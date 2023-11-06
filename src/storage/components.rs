use crate::components::{celestial_body_component::CelestialBodyComponent, mass_component::MassComponent, parent_component::ParentComponent, position_component::PositionComponent, trajectory_component::TrajectoryComponent, velocity_component::VelocityComponent};

use super::{entity_allocator::EntityAllocator, component_storage::ComponentStorage};

pub struct Components {
    pub entity_allocator: EntityAllocator,
    pub celestial_body_components: ComponentStorage<CelestialBodyComponent>,
    pub mass_components: ComponentStorage<MassComponent>,
    pub parent_components: ComponentStorage<ParentComponent>,
    pub position_components: ComponentStorage<PositionComponent>,
    pub trajectory_components: ComponentStorage<TrajectoryComponent>,
    pub velocity_components: ComponentStorage<VelocityComponent>,
}

impl Components {
    pub fn new() -> Self {
        Self { 
            entity_allocator: EntityAllocator::new() ,
            celestial_body_components: ComponentStorage::new(),
            mass_components: ComponentStorage::new(),
            parent_components: ComponentStorage::new(),
            position_components: ComponentStorage::new(),
            trajectory_components: ComponentStorage::new(),
            velocity_components: ComponentStorage::new(),
        }
    }
}