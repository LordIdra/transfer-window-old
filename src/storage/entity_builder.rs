use eframe::epaint::Rgba;
use nalgebra_glm::DVec2;

use crate::{components::{celestial_body_component::CelestialBodyComponent, mass_component::MassComponent, parent_component::ParentComponent, position_component::PositionComponent, trajectory_component::TrajectoryComponent, velocity_component::VelocityComponent, name_component::NameComponent, Components, icon_component::{IconComponent, IconType}}, storage::entity_allocator::Entity};

struct EntityBuilder {
    celestial_body_component: Option<CelestialBodyComponent>,
    icon_component: Option<IconComponent>,
    mass_component: Option<MassComponent>,
    name_component: Option<NameComponent>,
    parent_component: Option<ParentComponent>,
    position_component: Option<PositionComponent>,
    trajectory_component: Option<TrajectoryComponent>,
    velocity_component: Option<VelocityComponent>,
}

impl EntityBuilder {
    pub fn new() -> Self {
        Self { 
            celestial_body_component: None, 
            icon_component: None,
            mass_component: None, 
            name_component: None,
            parent_component: None, 
            position_component: None, 
            trajectory_component: None, 
            velocity_component: None 
        }
    }

    pub fn with_celestial_body_component(mut self, component: CelestialBodyComponent) -> Self {
        self.celestial_body_component = Some(component);
        self
    }

    pub fn with_icon_component(mut self, component: IconComponent) -> Self {
        self.icon_component = Some(component);
        self
    }

    pub fn with_mass_component(mut self, component: MassComponent) -> Self {
        self.mass_component = Some(component);
        self
    }

    pub fn with_name_component(mut self, component: NameComponent) -> Self {
        self.name_component = Some(component);
        self
    }

    pub fn with_parent_component(mut self, component: ParentComponent) -> Self {
        self.parent_component = Some(component);
        self
    }

    pub fn with_position_component(mut self, component: PositionComponent) -> Self {
        self.position_component = Some(component);
        self
    }

    pub fn with_trajectory_component(mut self, component: TrajectoryComponent) -> Self {
        self.trajectory_component = Some(component);
        self
    }

    pub fn with_velocity_component(mut self, component: VelocityComponent) -> Self {
        self.velocity_component = Some(component);
        self
    }

    pub fn build(self, components: &mut Components) -> Entity {
        let EntityBuilder {
            celestial_body_component,
            icon_component,
            mass_component,
            name_component,
            parent_component,
            position_component,
            trajectory_component,
            velocity_component,
        } = self;

        let entity = components.entity_allocator.allocate();
        components.celestial_body_components.set(entity, celestial_body_component);
        components.icon_components.set(entity, icon_component);
        components.mass_components.set(entity, mass_component);
        components.name_components.set(entity, name_component);
        components.parent_components.set(entity, parent_component);
        components.position_components.set(entity, position_component);
        components.trajectory_components.set(entity, trajectory_component);
        components.velocity_components.set(entity, velocity_component);
        entity
    }
}

fn base_object_builder(type_name: String, name: String, absolute_position: DVec2, velocity: DVec2, mass: f64) -> EntityBuilder {
    let icon_size = 0.01;
    EntityBuilder::new()
        .with_name_component(NameComponent::new(name))
        .with_icon_component(IconComponent::new(absolute_position, IconType::ObjectIcon, type_name, icon_size))
        .with_position_component(PositionComponent::new(absolute_position))
        .with_velocity_component(VelocityComponent::new(velocity))
        .with_mass_component(MassComponent::new(mass))
}

pub fn add_root_object(components: &mut Components, type_name: String, name: String, position: DVec2, velocity: DVec2, mass: f64, radius: f64, color: Rgba) -> Entity {
    base_object_builder(type_name, name, position, velocity, mass)
        .with_celestial_body_component(CelestialBodyComponent::new(radius, color))
        .build(components)
}

pub fn add_child_celestial_object(components: &mut Components, time: f64, type_name: String, name: String, parent: Entity, position: DVec2, velocity: DVec2, mass: f64, radius: f64, color: Rgba) -> Entity {
    let absolute_position = components.position_components.get(&parent).unwrap().get_absolute_position() + position;
    let entity = base_object_builder(type_name, name, absolute_position, velocity, mass)
        .with_parent_component(ParentComponent::new(parent))
        .with_celestial_body_component(CelestialBodyComponent::new(radius, color))
        .with_trajectory_component(TrajectoryComponent::new(components, parent, position, velocity, time))
        .build(components);
    components.parent_components.set(entity, Some(ParentComponent::new(parent)));
    components.celestial_body_components
        .get_mut(&parent)
        .expect("Object's parent must be a celestial body")
        .add_child(entity);
    entity
}

pub fn add_child_object(components: &mut Components, time: f64, type_name: String, name: String, parent: Entity, position: DVec2, velocity: DVec2, mass: f64) -> Entity {
    let absolute_position = components.position_components.get(&parent).unwrap().get_absolute_position() + position;
    let entity = base_object_builder(type_name, name, absolute_position, velocity, mass)
        .with_parent_component(ParentComponent::new(parent))
        .with_trajectory_component(TrajectoryComponent::new(components, parent, position, velocity, time))
        .build(components);
    components.parent_components.set(entity, Some(ParentComponent::new(parent)));
    components.celestial_body_components
        .get_mut(&parent)
        .expect("Object's parent must be a celestial body")
        .add_child(entity);
    entity
}
