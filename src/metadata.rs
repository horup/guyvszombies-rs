use macroquad::texture::{load_texture, Texture2D};
use std::{collections::HashMap, default};
pub type AssetIndex = u16;
pub struct ImageInfo {
    pub index: AssetIndex,
    pub name: String,
    pub path: String,
    pub texture: Texture2D,
}

#[derive(Clone)]
pub struct FrameIndex {
    pub image:AssetIndex,
    pub frame:u16
}
#[derive(Clone, Default)]
pub struct ActorInfo {
    pub index: AssetIndex,
    pub name: String,
    pub frames: Vec<FrameIndex>,
}

impl Asset for ActorInfo {
    fn name(&self) -> &str {
        &self.name
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn set_index(&mut self, index: u16) {
        self.index = index;
    }
}

impl Asset for ImageInfo {
    fn name(&self) -> &str {
        &self.name
    }

    fn index(&self) -> AssetIndex {
        self.index
    }

    fn set_index(&mut self, index: AssetIndex) {
        self.index = index;
    }
}

pub struct Assets<T> {
    inner: Vec<T>,
    name_to_index: HashMap<String, AssetIndex>,
}

impl<T> Default for Assets<T> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
            name_to_index: Default::default(),
        }
    }
}

impl<T> Assets<T> {
    pub fn get(&self, index: AssetIndex) -> Option<&T> {
        self.inner.get(index as usize)
    }
}

impl<T: Asset> Assets<T> {
    pub fn find(&self, name: &str) -> Option<&T> {
        let Some(index) = self.name_to_index.get(name) else {
            return None;
        };
        self.get(*index)
    }

    pub fn push(&mut self, mut t: T) {
        let index = self.inner.len() as AssetIndex;
        self.name_to_index.insert(t.name().to_string(), index);
        t.set_index(index);
        self.inner.push(t);
    }
}

impl Assets<ImageInfo> {
    pub async fn read_from(&mut self, table: toml::Table) {
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
    pub async fn read_from(&mut self, table: toml::Table, images: &Assets<ImageInfo>) {
        for (name, props) in table {
            let get_string = |x: &str| {
                let Some(v) = props.get(x) else {
                    return None;
                };
                v.as_str()
            };

            let get_f32 = |x: &str| {
                let Some(v) = props.get(x) else {
                    return None;
                };
                v.as_float()
                    .or(v.as_integer().map(|x| x as f64))
                    .map(|x| x as f32)
            };

            let get_i32 = |x: &str| {
                let Some(v) = props.get(x) else {
                    return None;
                };
                v.as_integer()
            };

            let get_array_string = |x: &str| {
                let Some(v) = props.get(x) else {
                    return None;
                };

                let mut res = Vec::new();
                let Some(v) = v.as_array() else {
                    return None;
                };
                for v in v.iter() {
                    let Some(v) = v.as_str() else {
                        return None;
                    };
                    res.push(v.to_string());
                }

                Some(res)
            };

            let extends = get_string("extends");
            let mut actor_info: ActorInfo = match extends {
                Some(extends) => self
                    .find(extends)
                    .expect("could not find base actor to extend from")
                    .clone(),
                None => ActorInfo::default(),
            };
            actor_info.name = name.clone();
            actor_info.frames = match get_array_string("frames") {
                Some(frames) => frames
                    .iter()
                    .map(|frame| FrameIndex { image: images.find(&frame).expect("frame was not found").index, frame: 0 })
                    .collect(),
                None => actor_info.frames,
            };


            self.push(actor_info);
        }
    }
}

pub trait Asset {
    fn name(&self) -> &str;
    fn index(&self) -> u16;
    fn set_index(&mut self, index: u16);
}

#[derive(Default)]
pub struct Metadata {
    pub images: Assets<ImageInfo>,
    pub actors: Assets<ActorInfo>,
}
