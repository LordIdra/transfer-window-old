use eframe::epaint::Rgba;
use nalgebra_glm::DVec2;

use crate::{components::{celestial_body_component::CelestialBodyComponent, mass_component::MassComponent, parent_component::ParentComponent, position_component::PositionComponent, trajectory_component::TrajectoryComponent, velocity_component::VelocityComponent, name_component::NameComponent, sphere_of_influence_component::SphereOfInfluenceComponent}, storage::{entity_allocator::Entity, components::Components}};

struct EntityBuilder {
    celestial_body_component: Option<CelestialBodyComponent>,
    mass_component: Option<MassComponent>,
    name_component: Option<NameComponent>,
    parent_component: Option<ParentComponent>,
    position_component: Option<PositionComponent>,
    sphere_of_influence_component: Option<SphereOfInfluenceComponent>,
    trajectory_component: Option<TrajectoryComponent>,
    velocity_component: Option<VelocityComponent>,
}

impl EntityBuilder {
    pub fn new() -> Self {
        Self { 
            celestial_body_component: None, 
            mass_component: None, 
            name_component: None,
            parent_component: None, 
            position_component: None, 
            sphere_of_influence_component: None,
            trajectory_component: None, 
            velocity_component: None 
        }
    }

    pub fn with_celestial_body_component(mut self, component: CelestialBodyComponent) -> Self {
        self.celestial_body_component = Some(component);
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

    pub fn with_sphere_of_influence_component(mut self, component: SphereOfInfluenceComponent) -> Self {
        self.sphere_of_influence_component = Some(component);
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

    pub fn build(&self, components: &mut Components) -> Entity {
        let entity = components.entity_allocator.allocate();
        if let Some(component) = self.celestial_body_component {
            components.celestial_body_components.set(entity, component);
        }
        if let Some(component) = self.mass_component {
            components.mass_components.set(entity, component);
        }
        if let Some(component) = self.parent_component {
            components.parent_components.set(entity, component);
        }
        if let Some(component) = self.position_component {
            components.position_components.set(entity, component);
        }
        if let Some(component) = self.trajectory_component {
            components.trajectory_components.set(entity, component);
        }
        if let Some(component) = self.velocity_component {
            components.velocity_components.set(entity, component);
        }
        entity
    }
}

pub fn add_root_object(components: &mut Components, name: String, absolute_position: DVec2, velocity: DVec2, mass: f64, radius: f64, color: Rgba) -> Entity {
    EntityBuilder::new()
        .with_name_component(NameComponent::new(name))
        .with_position_component(PositionComponent::new(absolute_position))
        .with_velocity_component(VelocityComponent::new(velocity))
        .with_mass_component(MassComponent::new(mass))
        .with_celestial_body_component(CelestialBodyComponent::new(radius, color))
        .build(components)
}

pub fn add_child_object(components: &mut Components, name: String, parent: Entity, absolute_position: DVec2, velocity: DVec2, mass: f64, radius: f64, color: Rgba) -> Entity {
    let entity = add_root_object(components, name, absolute_position, velocity, mass, radius, color);
    components.parent_components.set(entity, ParentComponent::new(parent));
    components.celestial_body_components
        .get_mut(&parent)
        .expect("Object's parent must be a celestial body")
        .add_child(entity);
    entity
}