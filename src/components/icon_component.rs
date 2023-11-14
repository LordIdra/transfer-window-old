pub enum IconState {
    None,
    Hovered,
    Selected,
}

pub struct IconComponent {
    visible: bool,
    state: IconState,
    icon_name: String,
    icon_size: f64,
}

impl IconComponent {
    pub fn new(icon_name: String, icon_size: f64) -> Self {
        let shown = true;
        let state = IconState::None;
        Self { visible: shown, state, icon_name, icon_size }
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

    pub fn get_icon_name(&self) -> &String {
        &self.icon_name
    }

    pub fn get_icon_size(&self, zoom: f64) -> f64 {
        self.icon_size / zoom
    }
}