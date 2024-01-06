use std::{cell::RefCell, rc::Weak};

use eframe::epaint::Rgba;
use nalgebra_glm::{DVec2, vec2};

use super::trajectory_component::segment::burn::Burn;

const BURN_ARROW_DISPLACEMENT: f64 = 3.0e4;

#[derive(Clone, Debug)]
pub enum BurnArrowIconType {
    FRONT,
    RIGHT,
    BACK,
    LEFT,
}

impl BurnArrowIconType {
    pub fn get_relative_position(&self, forward_unit: DVec2, zoom: f64) -> DVec2 {
        let unadjusted = match self {
            BurnArrowIconType::FRONT => forward_unit * BURN_ARROW_DISPLACEMENT,
            BurnArrowIconType::RIGHT => vec2(forward_unit.y, -forward_unit.x) * BURN_ARROW_DISPLACEMENT,
            BurnArrowIconType::BACK => -forward_unit * BURN_ARROW_DISPLACEMENT,
            BurnArrowIconType::LEFT => -vec2(forward_unit.y, -forward_unit.x) * BURN_ARROW_DISPLACEMENT,
        };
        unadjusted / zoom
    }
}

#[derive(Clone)]
pub enum IconType {
    ObjectIcon,
    BurnIcon(Weak<RefCell<Burn>>),
    BurnArrowIcon(Weak<RefCell<Burn>>, BurnArrowIconType),
}

impl IconType {
    pub fn takes_precedence_over(&self, other: IconType) -> Option<bool> {
        match self {
            IconType::ObjectIcon => match other {
                IconType::ObjectIcon => Some(true),
                IconType::BurnIcon(_) => Some(true),
                IconType::BurnArrowIcon(_, _) => None,
            },
            IconType::BurnIcon(_) => match other {
                IconType::ObjectIcon => Some(false),
                IconType::BurnIcon(_) => Some(true),
                IconType::BurnArrowIcon(_, _) => None,
            },
            IconType::BurnArrowIcon(_, _) => None,
        }
    }

    pub fn as_burn_icon(&self) -> Weak<RefCell<Burn>> {
        let IconType::BurnIcon(burn) = self else {
            panic!("Attempt to get non-burn-icon as burn icon")
        };
        burn.clone()
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
    facing: Option<DVec2>,
}

impl IconComponent {
    pub fn new(_type: IconType, color: Rgba, name: String, size: f64, facing: Option<DVec2>) -> Self {
        let shown = true;
        let state = IconState::None;
        Self { visible: shown, state, _type, color, name, size, facing }
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_state(&mut self, state: IconState) {
        self.state = state;
    }

    pub fn set_facing(&mut self, facing: DVec2) {
        self.facing = Some(facing);
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

    pub fn get_facing(&self) -> Option<DVec2> {
        self.facing
    }
}