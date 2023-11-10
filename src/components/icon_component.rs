use eframe::epaint::Rgba;

pub enum IconState {
    None,
    Hovered,
    Selected,
}

pub struct IconComponent {
    state: IconState,
    icon_name: String,
    icon_color: Rgba,
    icon_size: f64,
}

impl IconComponent {
    pub fn new(icon_name: String, icon_color: Rgba, icon_size: f64) -> Self {
        let state = IconState::None;
        Self { state, icon_name, icon_color, icon_size }
    }
}