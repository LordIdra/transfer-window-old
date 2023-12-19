use std::{cell::RefCell, rc::Rc};

use eframe::epaint::Rgba;

use super::trajectory_component::segment::burn::Burn;

#[derive(Clone)]
pub enum IconType {
    ObjectIcon,
    BurnIcon(Rc<RefCell<Burn>>),
}

impl IconType {
    pub fn takes_precedence_over(&self, other: IconType) -> bool {
        match self {
            IconType::ObjectIcon => match other {
                IconType::ObjectIcon => true,
                IconType::BurnIcon(_) => true,
            },
            IconType::BurnIcon(_) => match other {
                IconType::ObjectIcon => false,
                IconType::BurnIcon(_) => true,
            },
        }
    }
}

pub enum IconState {
    None,
    Hovered,
    Selected,
}

pub struct IconComponent {
    visible: bool,
    state: IconState,
    _type: IconType,
    color: Rgba,
    name: String,
    size: f64,
}

impl IconComponent {
    pub fn new(_type: IconType, color: Rgba, name: String, size: f64) -> Self {
        let shown = true;
        let state = IconState::None;
        Self { visible: shown, state, _type, color, name, size }
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_state(&mut self, state: IconState) {
        self.state = state;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn get_state(&self) -> &IconState {
        &self.state
    }

    pub fn get_type(&self) -> IconType {
        self._type.clone()
    }

    pub fn get_color(&self) -> Rgba {
        self.color
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_size(&self, zoom: f64) -> f64 {
        self.size / zoom
    }
}