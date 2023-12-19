use macroquad::texture::{load_texture, Texture2D};
use toml::Value;
use std::{collections::HashMap, default};
pub type AssetIndex = u16;
pub struct ImageInfo {
    pub index: AssetIndex,
    pub name: String,
    pub path: String,
    pub texture: Texture2D,
}

#[derive(Clone, Copy)]
pub struct FrameIndex {
    pub image: AssetIndex,
    pub frame: u16,
}

#[derive(Clone, Default)] pub struct WeaponInfo {
    pub index:AssetIndex,
    pub rate_of_fire:f32,
    pub name:String,
    pub frames:Vec<FrameIndex>,
    pub damage:[f32;2]
}

impl Asset for WeaponInfo {
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

#[derive(Clone, Default)]
pub struct ActorInfo {
    pub index: AssetIndex,
    pub name: String,
    pub frames: Vec<FrameIndex>,
    pub locomotion_frames: Vec<FrameIndex>,
    pub dead_frames: Vec<FrameIndex>,
    pub bot: bool,
    pub speed: f32,
    pub radius:f32,
    pub missile:bool,
    pub shootable:bool,
    pub health:f32,
    pub solid:bool,
    pub particle:bool,
    pub weapon:AssetIndex
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
            texture.set_filter(macroquad::miniquad::FilterMode::Nearest);
            self.push(ImageInfo {
                index: 0,
                name: key.clone(),
                path: path.clone(),
                texture: texture,
            });
        }
    }
}

fn get_f32(prop:&str, props:&Value) -> Option<f32> {
    let Some(v) = props.get(prop) else {
        return None;
    };
    v.as_float()
        .or(v.as_integer().map(|x| x as f64))
        .map(|x| x as f32)
}
fn get_array_string(prop:&str, props:&Value) -> Option<Vec<String>> {
    let Some(v) = props.get(prop) else {
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
}

fn get_bool(prop:&str, props:&Value) -> Option<bool> {
    let Some(v) = props.get(prop) else {
        return None;
    };
    v.as_bool()
}

fn get_str<'a>(prop:&'a str, props:&'a Value) -> Option<&'a str> {
    let Some(v) = props.get(prop) else {
        return None;
    };
    v.as_str()
}

fn get_array_f32<'a>(prop:&'a str, props:&'a Value) -> Option<Vec<f32>> {
    let Some(v) = props.get(prop) else { return None};
    let Some(v) = v.as_array() else { return None; };
    let mut vec = Vec::new();
    for v in v.iter() {
        match v {
            Value::Integer(i) => {vec.push(*i as f32)},
            Value::Float(f) => {vec.push(*f as f32)},
            _=> {}
        }
    }
    return Some(vec);
}

impl Assets<WeaponInfo> {
    pub async fn read_from(&mut self, table: toml::Table, images: &Assets<ImageInfo>) {
        for (name, props ) in table {
            let extends = get_str("extends", &props);
            let base: WeaponInfo = match extends {
                Some(extends) => self
                    .find(extends)
                    .expect("could not find base weapon to extend from")
                    .clone(),
                None => WeaponInfo::default(),
            };
            let mut weapon_info = WeaponInfo::default();
            weapon_info.name = name.clone();
            let frames = match get_array_string("frames", &props) {
                Some(frames) => frames
                    .iter()
                    .map(|frame| FrameIndex {
                        image: images.find(&frame).expect("frame was not found").index,
                        frame: 0,
                    })
                    .collect(),
                None => base.frames,
            };
            let rate_of_fire = get_f32("rate_of_fire", &props).unwrap_or(base.rate_of_fire);
            let damage = match get_array_f32("damage", &props) {
                Some(damage) => [damage.get(0).copied().unwrap_or_default(), damage.get(1).copied().unwrap_or_default()],
                None => base.damage,
            };
            
            let  weapon_info = WeaponInfo {
                index: 0,
                rate_of_fire,
                name,
                frames,
                damage,
            };
            self.push(weapon_info)
        }
    }
}

impl Assets<ActorInfo> {
    pub async fn read_from(&mut self, table: toml::Table, images: &Assets<ImageInfo>, weapons: &Assets<WeaponInfo>) {
        for (name, props) in table {
           
            let extends = get_str("extends", &props);
            let base: ActorInfo = match extends {
                Some(extends) => self
                    .find(extends)
                    .expect("could not find base actor to extend from")
                    .clone(),
                None => ActorInfo::default(),
            };
            let frames = match get_array_string("frames", &props) {
                Some(frames) => frames
                    .iter()
                    .map(|frame| FrameIndex {
                        image: images.find(&frame).expect("frame was not found").index,
                        frame: 0,
                    })
                    .collect(),
                None => base.frames,
            };
            let locomotion_frames = match get_array_string("locomotion_frames", &props) {
                Some(frames) => frames
                    .iter()
                    .map(|frame| FrameIndex {
                        image: images.find(&frame).expect("frame was not found").index,
                        frame: 0,
                    })
                    .collect(),
                None => base.locomotion_frames,
            };
            let dead_frames = match get_array_string("dead_frames", &props) {
                Some(frames) => frames
                    .iter()
                    .map(|frame| FrameIndex {
                        image: images.find(&frame).expect("frame was not found").index,
                        frame: 0,
                    })
                    .collect(),
                None => base.dead_frames,
            };
            let weapon = match get_str("weapon", &props).and_then(|x|weapons.find(x)) {
                Some(wp) => wp.index,
                None => base.weapon,
            };
            
            let actor_info = ActorInfo {
                index: 0,
                name: name.clone(),
                frames,
                locomotion_frames:locomotion_frames,
                bot: get_bool("bot", &props).unwrap_or(base.bot),
                speed: get_f32("speed", &props).unwrap_or(base.speed),
                radius: get_f32("radius", &props).unwrap_or(base.radius),
                missile: get_bool("missile", &props).unwrap_or(base.missile),
                shootable: get_bool("shootable", &props).unwrap_or(base.shootable),
                health: get_f32("health", &props).unwrap_or(base.health),
                solid: get_bool("solid", &props).unwrap_or(base.solid),
                particle:get_bool("particle", &props).unwrap_or(base.particle),
                weapon,
                dead_frames
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
    pub weapons: Assets<WeaponInfo>,
}
