use std::{fs::{self, DirEntry}, sync::Arc};

use eframe::{epaint::ahash::HashMap, egui::ImageSource};
use glow::Context;

use crate::rendering::texture;

struct Texture {
    pub bytes: Vec<u8>,
    pub image: ImageSource<'static>,
    pub gl_texture: Option<texture::Texture>,
}

pub struct Resources {
    textures: HashMap<String, Texture>,
}

impl Resources {
    pub fn new() -> Self {
        let textures = fs::read_dir("resources/textures")
            .expect("Unable to find textures directory")
            .map(|entry| entry.expect("Failed to read file"))
            .map(|entry| (Self::get_entry_name(&entry), entry))
            .map(|entry| (entry.0, Self::load_texture(entry.1)))
            .collect();
        Resources { textures }
    }

    fn get_entry_name(entry: &DirEntry) -> String {
        entry.file_name().into_string().unwrap()
    }

    fn load_texture(entry: DirEntry) -> Texture {
        let uri = "file://".to_owned() + entry.path().as_path().as_os_str().to_str().unwrap();
        let bytes = fs::read(entry.path()).expect("Failed to load file");
        let image = image::load_from_memory(&bytes).expect("Failed to load image");
        let bytes = image.to_rgba8().into_vec();
        let image = ImageSource::Uri(uri.into());
        let gl_texture = None;
        Texture { bytes, image, gl_texture }
    }

    pub fn get_texture_bytes(&self, name: &str) -> &Vec<u8> {
        &self.textures.get(name).unwrap_or_else(|| panic!("Texture {} does not exist", name)).bytes
    }

    pub fn get_texture_image(&self, name: &str) -> ImageSource {
        self.textures.get(name).unwrap_or_else(|| panic!("Texture {} does not exist", name)).image.clone()
    }

    pub fn get_gl_texture(&self, gl: Arc<Context>, name: &str) -> &texture::Texture {
        let texture = self.textures.get(name).unwrap_or_else(|| panic!("Texture {} does not exist", name));
        if let Some(gl_texture) = &texture.gl_texture {
            return gl_texture;
        }
        todo!()
    }
}