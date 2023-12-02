use std::collections::HashMap;

use macroquad::{texture::{load_texture, Texture2D}};

pub struct ImageInfo {
    pub index:u16,
    pub name:String,
    pub path:String,
    pub texture:Texture2D
}

impl Asset for ImageInfo {
    fn name(&self) -> &str {
        &self.name
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn set_index(&mut self, index:u16) {
        self.index = index;
    }
}

pub struct Assets<T> {
    inner:Vec<T>,
    name_to_index:HashMap<String, u16>
}

impl<T> Default for Assets<T> {
    fn default() -> Self {
        Self { inner: Default::default(), name_to_index: Default::default() }
    }
}

impl<T> Assets<T> {
    pub fn get(&self, index:u16) -> Option<&T> {
        self.inner.get(index as usize)
    }
}

impl<T:Asset> Assets<T> {
    pub fn find(&self, name:&str) -> Option<&T> {
        let Some(index) = self.name_to_index.get(name) else { return None;};
        self.get(*index)
    }

    pub fn push(&mut self, mut t:T) {
        let index = self.inner.len() as u16;
        self.name_to_index.insert(t.name().to_string(), index);
        t.set_index(index);
        self.inner.push(t);
    }
}

impl Assets<ImageInfo> {
    pub async fn read_from(&mut self, table:toml::Table) {
        for (key, v) in table.iter() {
            let path = v.as_str().unwrap();
            let path = "assets/".to_string() + path;
            let texture = load_texture(&path).await.expect("failed to load texture");
            self.push(ImageInfo {
                index: 0,
                name: key.clone(),
                path: path.clone(),
                texture: texture,
            });
        }
    }
}

pub trait Asset {
    fn name(&self) -> &str;
    fn index(&self) -> u16;
    fn set_index(&mut self, index:u16);
}

#[derive(Default)]
pub struct Metadata {
    pub images:Assets<ImageInfo>
}

