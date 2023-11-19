use crate::storage::{entity_allocator::{EntityAllocator, Entity}, index_storage::ComponentStorage};

use self::{celestial_body_component::CelestialBodyComponent, mass_component::MassComponent, parent_component::ParentComponent, position_component::PositionComponent, trajectory_component::TrajectoryComponent, velocity_component::VelocityComponent, name_component::NameComponent, icon_component::IconComponent};

pub mod celestial_body_component;
pub mod icon_component;
pub mod mass_component;
pub mod name_component;
pub mod parent_component;
pub mod position_component;
pub mod trajectory_component;
pub mod velocity_component;

pub struct Components {
    pub entity_allocator: EntityAllocator,
    pub celestial_body_components: ComponentStorage<CelestialBodyComponent>,
    pub icon_components: ComponentStorage<IconComponent>,
    pub mass_components: ComponentStorage<MassComponent>,
    pub name_components: ComponentStorage<NameComponent>,
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
            icon_components: ComponentStorage::new(),
            mass_components: ComponentStorage::new(),
            name_components: ComponentStorage::new(),
            parent_components: ComponentStorage::new(),
            position_components: ComponentStorage::new(),
            trajectory_components: ComponentStorage::new(),
            velocity_components: ComponentStorage::new(),
        }
    }
}