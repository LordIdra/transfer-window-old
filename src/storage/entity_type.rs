use crate::components::icon_component::IconComponent;

pub enum EntityType {
    Star,
    Planet,
    Moon,
    Spacecraft,
}

impl EntityType {
    pub fn get_icon_component(&self) -> IconComponent {
        match self {
            EntityType::Star => todo!(),//IconComponent::new("star", icon_color, icon_size),
            EntityType::Planet => todo!(),
            EntityType::Moon => todo!(),
            EntityType::Spacecraft => todo!(),
        }
    }
}