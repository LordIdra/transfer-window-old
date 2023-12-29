use crate::{storage::entity_allocator::Entity, state::State};

pub fn get_entity_by_name(state: &mut State, name: String) -> Entity {
    for entity in state.components.entity_allocator.get_entities() {
        if let Some(name_component) = state.components.name_components.get(&entity) {
            if name_component.get_name() == name {
                return entity;
            };
        }
    }
    panic!();
}

pub fn format_time(time: f64) -> String {
    let years_quotient = f64::floor(time / (360.0 * 24.0 * 60.0 * 60.0));
    let years_remainder = time % (360.0 * 24.0 * 60.0 * 60.0);
    let days_quotient = f64::floor(years_remainder / (24.0 * 60.0 * 60.0));
    let days_remainder = years_remainder % (24.0 * 60.0 * 60.0);
    let hours_quotient = f64::floor(days_remainder / (60.0 * 60.0));
    let hours_remainder = days_remainder % (60.0 * 60.0);
    let minutes_quotient = f64::floor(hours_remainder / 60.0);
    let seconds = f64::round(hours_remainder % 60.0);
    if years_quotient != 0.0 {
        "".to_string()
            + years_quotient.to_string().as_str() + "y"
            + days_quotient.to_string().as_str() + "d"
            + hours_quotient.to_string().as_str() + "h"
            + minutes_quotient.to_string().as_str() + "m"
            + seconds.to_string().as_str() + "s"
    } else if days_quotient != 0.0 {
        "".to_string()
            + days_quotient.to_string().as_str() + "d"
            + hours_quotient.to_string().as_str() + "h"
            + minutes_quotient.to_string().as_str() + "m"
            + seconds.to_string().as_str() + "s"
    } else if hours_quotient != 0.0 {
        "".to_string()
            + hours_quotient.to_string().as_str() + "h"
            + minutes_quotient.to_string().as_str() + "m"
            + seconds.to_string().as_str() + "s"
    } else if minutes_quotient != 0.0 {
        "".to_string()
            + minutes_quotient.to_string().as_str() + "m"
            + seconds.to_string().as_str() + "s"
    } else {
        "".to_string()
            + seconds.to_string().as_str() + "s"
    }
}