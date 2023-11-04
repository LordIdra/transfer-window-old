use std::collections::HashSet;

use eframe::epaint::Color32;

pub struct CelestialBodyComponent {
    radius: f64,
    color: Color32,
    sphere_of_influence: f64,
    children: HashSet<Entity>,
}