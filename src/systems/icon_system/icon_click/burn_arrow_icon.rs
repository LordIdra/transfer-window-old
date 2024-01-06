use std::{rc::{Weak, Rc}, cell::RefCell};

use eframe::egui::{InputState, PointerButton};

use crate::{state::State, storage::entity_allocator::Entity, components::{icon_component::{IconState, BurnArrowIconType}, trajectory_component::segment::{burn::Burn, orbit::Orbit, Segment}}, systems::trajectory_prediction_system::spacecraft_prediction::predict_spacecraft};

pub fn select_burn_arrow_icon(state: &mut State, input: &InputState, burn: Weak<RefCell<Burn>>, type_: BurnArrowIconType) {
    if !input.pointer.button_down(PointerButton::Primary) {
        return;
    }

    let burn = burn.upgrade().unwrap();
    let entity = burn.borrow().get_entity();
    state.components.trajectory_components.get_mut(&entity).unwrap().remove_segments_starting_after_segment(Segment::Burn(burn.clone()));
    burn.borrow_mut().adjust(type_.get_adjustment() * 10.0);
    let end_point = burn.borrow().get_end_point().clone();
    let end_time = end_point.get_time();
    let orbit = Orbit::new(&state.components, burn.borrow().get_parent(), end_point.get_position(), end_point.get_velocity(), end_time);
    state.components.trajectory_components.get_mut(&entity).unwrap().add_segment(Segment::Orbit(Rc::new(RefCell::new(orbit))));
    predict_spacecraft(state, entity, end_time)
}

pub fn update_burn_arrow_icon(state: &mut State, mouse_over: &Option<Entity>, entity: &Entity) {
    // If burn is being hovered
    if let Some(mouse_over) = mouse_over {
        if *entity == *mouse_over {
            state.components.icon_components.get_mut(entity).unwrap().set_state(IconState::Hovered);
            return;
        }
    }

    // If burn is not being hovered and is not actively selected
    state.components.icon_components.get_mut(entity).unwrap().set_state(IconState::None)
}