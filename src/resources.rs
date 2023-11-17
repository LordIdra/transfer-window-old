use std::{fs::{self, DirEntry, ReadDir}, sync::Arc};

use eframe::{epaint::ahash::{HashMap, HashSet}, egui::ImageSource};
use glow::Context;
use image::GenericImageView;

use crate::rendering::texture;

struct Texture {
    pub size: (i32, i32),
    pub bytes: Vec<u8>,
    pub image: ImageSource<'static>,
    pub gl_texture: Option<texture::Texture>,
}

pub struct Resources {
    texture_names: Vec<String>,
    textures: HashMap<String, Texture>,
}

impl Resources {
    pub fn new() -> Self {
        let texture_names = Self::get_entries("resources/textures".to_string())
            .into_iter()
            .map(|entry| Self::get_entry_name(&entry))
            .collect();
        let textures = Self::get_entries("resources/textures".to_string())
            .into_iter()
            .map(|entry| (Self::get_entry_name(&entry), entry))
            .map(|entry| (entry.0, Self::load_texture(entry.1)))
            .collect();
        Resources { texture_names, textures }
    }

    fn get_entries(directory: String) -> Vec<DirEntry> {
        fs::read_dir(directory)
            .expect("Failed to read directory")
            .map(|entry| entry.expect("Failed to read file"))
            .collect()
    }

    fn get_entry_name(entry: &DirEntry) -> String {
        entry.file_name().into_string().unwrap().split('.').next().unwrap().to_string()
    }

    fn load_texture(entry: DirEntry) -> Texture {
        let uri = "file://".to_owned() + entry.path().as_path().as_os_str().to_str().unwrap();
        let bytes = fs::read(entry.path()).expect("Failed to load file");
        let image = image::load_from_memory(&bytes).expect("Failed to load image");
        let size = (image.dimensions().0 as i32, image.dimensions().1 as i32);
        let bytes = image.to_rgba8().into_vec();
        let image = ImageSource::Uri(uri.into());
        let gl_texture = None;
        Texture { size, bytes, image, gl_texture }
    }

    pub fn get_texture_names(&self) -> &Vec<String> {
        &self.texture_names
    }

    pub fn get_texture_image(&self, name: &str) -> ImageSource {
        self.textures.get(name).unwrap_or_else(|| panic!("Texture {} does not exist", name)).image.clone()
    }

    pub fn get_gl_texture(&mut self, gl: Arc<Context>, name: &str) -> &texture::Texture {
        let texture = self.textures.get_mut(name).unwrap_or_else(|| panic!("Texture {} does not exist", name));
        if texture.gl_texture.is_none() {
            texture.gl_texture = Some(texture::Texture::new(gl, texture.size, texture.bytes.clone()));
        }
        texture.gl_texture.as_ref().unwrap()
    }
}