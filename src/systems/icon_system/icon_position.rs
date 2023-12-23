use crate::{state::State, components::icon_component::IconType};

use self::{object_icon::update_object_icon_position, burn_icon::update_burn_icon_position, burn_arrow_icon::update_burn_arrow_icon_position};

mod burn_arrow_icon;
mod burn_icon;
mod object_icon;


pub fn icon_position(state: &mut State) {
    for entity in state.components.entity_allocator.get_entities() {
        if state.components.icon_components.get(&entity).is_some() {
            match state.components.icon_components.get(&entity).unwrap().get_type() {
                IconType::ObjectIcon => update_object_icon_position(state, entity),
                IconType::BurnIcon(burn) => update_burn_icon_position(state, entity, burn),
                IconType::BurnArrowIcon(burn, _type) => update_burn_arrow_icon_position(state, entity, burn, _type),
            }
        }
    }
}