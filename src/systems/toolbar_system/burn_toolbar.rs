use std::{rc::Rc, cell::RefCell};

use eframe::{egui::{Context, Window, Layout, Image, ImageButton, Ui, Style}, emath::{Align2, Align}, epaint::{self, Color32}};

use crate::{state::State, systems::{warp_update_system::WarpDescription, trajectory_prediction_system::spacecraft_prediction::predict_spacecraft, util::format_time}, components::trajectory_component::segment::burn::Burn, storage::entity_allocator::Entity};

use super::apply_toolbar_style;

fn warp_to_burn(state: &mut State, burn: Rc<RefCell<Burn>>) {
    let end_time = burn.borrow().get_start_point().get_time();
    state.current_warp = Some(WarpDescription::new(state.time, end_time));
}

fn delete_burn(state: &mut State, burn_icon: Entity, burn: Rc<RefCell<Burn>>) {
    let entity = burn.borrow().get_entity();
    let burn_start_time = burn.borrow().get_start_point().get_time();
    state.components.trajectory_components.get_mut(&entity).unwrap().remove_segments_after(burn_start_time);
    state.components.deallocate(burn_icon);
    state.selected_burn_icon = None;
    predict_spacecraft(state, entity, burn_start_time, 10000000.0);
}

fn draw(state: &mut State, ui: &mut Ui, burn_icon: Entity, burn: Rc<RefCell<Burn>>) {
    ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
        let warp_image = Image::new(state.resources.get_texture_image("warp-here"))
            .bg_fill(Color32::TRANSPARENT)
            .fit_to_exact_size(epaint::vec2(15.0, 15.0));
        let warp_button = ImageButton::new(warp_image);
        if ui.add(warp_button).clicked() {
            warp_to_burn(state, burn.clone());
        }

        let delete_image = Image::new(state.resources.get_texture_image("close"))
            .bg_fill(Color32::TRANSPARENT)
            .fit_to_exact_size(epaint::vec2(15.0, 15.0));
        let delete_button = ImageButton::new(delete_image);
        if ui.add(delete_button).clicked() {
            delete_burn(state, burn_icon, burn.clone());
        }
    });

    ui.label(format!("T- {}", format_time(burn.borrow().get_start_point().get_time() - state.time)));
    ui.label(format!("Delta-V: {:.0}", burn.borrow().get_total_dv()));

    state.register_ui(ui);
}


pub fn burn_toolbar(state: &mut State, context: &Context) {
    let Some(burn_icon) = state.selected_burn_icon.clone() else {
        return;
    };

    apply_toolbar_style(context);

    let burn = state.components.icon_components.get(&burn_icon).unwrap().get_type().as_burn_icon().upgrade().unwrap();
    let window = Window::new("Burn Toolbar")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0));
    window.show(context, |ui| draw(state, ui, burn_icon, burn));

    context.set_style(Style::default());
}