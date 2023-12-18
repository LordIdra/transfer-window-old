use nalgebra_glm::DVec2;

#[derive(Clone, Copy)]
pub enum IconType {
    ObjectIcon,
    BurnIcon,
}

#[derive(Clone, Copy)]
pub enum IconState {
    None,
    Hovered,
    Selected,
}

pub struct IconComponent {
    visible: bool,
    position: DVec2,
    state: IconState,
    icon_type: IconType,
    icon_name: String,
    icon_size: f64,
}

impl IconComponent {
    pub fn new(position: DVec2, icon_type: IconType, icon_name: String, icon_size: f64) -> Self {
        let shown = true;
        let state = IconState::None;
        Self { visible: shown, position, state, icon_type, icon_name, icon_size }
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_position(&mut self, position: DVec2) {
        self.position = position;
    }

    pub fn set_state(&mut self, state: IconState) {
        self.state = state;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn get_position(&self) -> DVec2 {
        self.position
    }

    pub fn get_state(&self) -> &IconState {
        &self.state
    }

    pub fn get_icon_type(&self) -> IconType {
        self.icon_type
    }

    pub fn get_icon_name(&self) -> &String {
        &self.icon_name
    }

    pub fn get_icon_size(&self, zoom: f64) -> f64 {
        self.icon_size / zoom
    }
}