pub struct NameComponent {
    name: String,
}

impl NameComponent {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}