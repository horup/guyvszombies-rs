use std::{collections::HashMap, default};
use macroquad::texture::{Texture2D, load_texture};
pub type AssetIndex = u16;
pub struct ImageInfo {
    pub index:AssetIndex,
    pub name:String,
    pub path:String,
    pub texture:Texture2D
}

#[derive(Default)]
pub struct ActorInfo {
    pub index:AssetIndex,
    pub name:String,
    pub frames:Vec<AssetIndex>
}

impl Asset for ImageInfo {
    fn name(&self) -> &str {
        &self.name
    }

    fn index(&self) -> AssetIndex {
        self.index
    }

    fn set_index(&mut self, index:AssetIndex) {
        self.index = index;
    }
}

pub struct Assets<T> {
    inner:Vec<T>,
    name_to_index:HashMap<String, AssetIndex>
}

impl<T> Default for Assets<T> {
    fn default() -> Self {
        Self { inner: Default::default(), name_to_index: Default::default() }
    }
}

impl<T> Assets<T> {
    pub fn get(&self, index:AssetIndex) -> Option<&T> {
        self.inner.get(index as usize)
    }
}

impl<T:Asset> Assets<T> {
    pub fn find(&self, name:&str) -> Option<&T> {
        let Some(index) = self.name_to_index.get(name) else { return None;};
        self.get(*index)
    }

    pub fn push(&mut self, mut t:T) {
        let index = self.inner.len() as AssetIndex;
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

impl Assets<ActorInfo> {
    pub async fn read_from(&mut self, table:toml::Table, images:&Assets<ImageInfo>) {
        for (name, props) in table {
            dbg!(name);
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
    pub images:Assets<ImageInfo>,
    pub actors:Assets<ActorInfo>
}

