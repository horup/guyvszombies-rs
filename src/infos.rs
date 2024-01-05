use std::{collections::HashMap, rc::Rc};

use glam::Vec2;
use macroquad::{
    file::load_file,
    texture::{load_texture, Texture2D, Image},
};
use toml::{Table, Value};

use crate::FrameIndex;

/// Extends the table with content from a 'super' table.
///
/// Assumes a key called 'extends' which points to a table from which it can extend
fn extend_table(mut tables: Table) -> Table {
    let names: Vec<String> = tables.keys().map(|x| x.to_owned()).collect();
    for name in names.iter() {
        let mut final_table = Table::default();
        let org_table = tables.get(name).unwrap().as_table().unwrap();
        if let Some(extends_table) = org_table
            .get("extends")
            .and_then(|x| x.as_str())
            .and_then(|x| tables.get(x))
            .and_then(|x| x.as_table())
        {
            final_table = extends_table.clone();
        }

        for (key, value) in org_table.iter() {
            final_table.insert(key.clone(), value.clone());
        }

        tables.insert(name.clone(), toml::Value::Table(final_table));
    }

    return tables;
}

/// Load table from path and extend it using the `extend_table` function
async fn load_and_extend_table(path: &str) -> Table {
    let table = load_table(path).await;
    extend_table(table)
}

/// Load table from path
async fn load_table(path: &str) -> Table {
    let table = String::from_utf8(load_file(path).await.unwrap()).unwrap();
    toml::from_str(&table).unwrap()
}

type InfoCollection<T> = HashMap<String, Rc<T>>;

pub struct ImageInfo2 {
    pub name: String,
    pub path: String,
    pub texture: Texture2D,
}

#[derive(Clone)]
pub struct ImageIndex {
    pub image: Rc<ImageInfo2>,
    pub frame: u16,
}

async fn load_images(table: &Table) -> InfoCollection<ImageInfo2> {
    let mut map = HashMap::default();
    for (name, value) in table.iter() {
        let Some(path) = value.as_str() else { continue; };
        let texture = load_texture(&("assets/".to_owned() + path)).await.expect("failed to load texture");
        map.insert(
            name.to_owned(),
            Rc::new(ImageInfo2 {
                name: name.to_owned(),
                path: path.to_owned(),
                texture,
            }),
        );
    }
    map
}

#[derive(Clone, Default)]
pub struct WeaponInfo2 {
    pub name:String,
    pub rate_of_fire: f32,
    pub frames: Vec<ImageIndex>,
    pub damage: [f32; 2],
    pub mount_offset: f32,
    pub muzzle_offset: f32,
    pub spread: f32,
}

fn get_f32(prop: &str, props: &Value) -> Option<f32> {
    let Some(v) = props.get(prop) else {
        return None;
    };
    v.as_float()
        .or(v.as_integer().map(|x| x as f64))
        .map(|x| x as f32)
}
fn get_array_string(prop: &str, props: &Value) -> Option<Vec<String>> {
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

fn get_bool(prop: &str, props: &Value) -> Option<bool> {
    let Some(v) = props.get(prop) else {
        return None;
    };
    v.as_bool()
}

fn get_str<'a>(prop: &'a str, props: &'a Value) -> Option<&'a str> {
    let Some(v) = props.get(prop) else {
        return None;
    };
    v.as_str()
}

fn get_array_f32<'a>(prop: &'a str, props: &'a Value) -> Option<Vec<f32>> {
    let Some(v) = props.get(prop) else {
        return None;
    };
    let Some(v) = v.as_array() else {
        return None;
    };
    let mut vec = Vec::new();
    for v in v.iter() {
        match v {
            Value::Integer(i) => vec.push(*i as f32),
            Value::Float(f) => vec.push(*f as f32),
            _ => {}
        }
    }
    return Some(vec);
}

fn get_vec2<'a>(prop: &'a str, props: &'a Value) -> Option<Vec2> {
    let Some(v) = get_array_f32(prop, props) else {
        return None;
    };
    if v.len() == 2 {
        return Some(Vec2::new(v[0], v[1]));
    }

    return None;
}

fn get_tuple_f32<'a>(prop: &'a str, props: &'a Value) -> Option<(f32, f32)> {
    let Some(v) = get_array_f32(prop, props) else {
        return None;
    };
    if v.len() == 2 {
        return Some((v[0], v[1]));
    }

    return None;
}

fn get_frames<'a>(prop: &'a str, props: &'a Value, images:&InfoCollection<ImageInfo2>) -> Vec<ImageIndex> {
    let mut frames = Vec::new();
    if let Some(props_frames) = get_array_string("frames", props) {
        for frame in props_frames.iter() {
            frames.push(ImageIndex {
                image: images.get(frame).expect("could not find image").clone(),
                frame: 0,
            });
            
        }
    }
    frames
}

async fn load_weapons(table:&Table, images:&InfoCollection<ImageInfo2>) -> HashMap<String, Rc<WeaponInfo2>> {
    let mut map = InfoCollection::default();
    
    for (name, props) in table.iter() {
        let damage = match get_array_f32("damage", &props) {
            Some(damage) => [
                damage.get(0).copied().unwrap_or_default(),
                damage.get(1).copied().unwrap_or_default(),
            ],
            None => [0.0, 0.0],
        };

        map.insert(name.to_owned(), Rc::new(WeaponInfo2 {
            name: name.to_owned(),
            rate_of_fire: get_f32("rate_of_fire", props).unwrap_or_default(),
            frames: get_frames("frames", props, images),
            damage: damage,
            mount_offset: get_f32("mount_offset", props).unwrap_or_default(),
            muzzle_offset: get_f32("muzzle_offset", props).unwrap_or_default(),
            spread: get_f32("spread", props).unwrap_or_default(),
        }));
    }
    map
}

#[derive(Clone)]
pub struct ActorInfo2 {
    pub name: String,
    pub frames: Vec<ImageIndex>,
    pub locomotion_frames: Vec<ImageIndex>,
    pub dead_frames: Vec<ImageIndex>,
    pub bot: bool,
    pub speed: f32,
    pub radius: f32,
    pub missile: bool,
    pub shootable: bool,
    pub health: f32,
    pub solid: bool,
    pub particle: bool,
    /// current active weapon
    pub weapon: Rc<WeaponInfo2>,
    /// frame offset from center of actor
    pub offset: Vec2,
    /// rotate the frame such that it faces facing
    pub rotate_to_face: bool,
    pub missile_direct_damage: (f32, f32),
    pub missile_splash_damage: (f32, f32),
}

pub struct Infos {
    // pub images:HashMap<String, ImageInfo2>
    //pub actors:HashMap<String, ActorInfo2>
}
impl Infos {
    pub async fn init() -> Self {
        let images = load_table("assets/images.toml").await;
        let images = load_images(&images).await;
        let weapons = load_and_extend_table("assets/weapons.toml").await;
        let weapons = load_weapons(&weapons, &images).await;
        let actors = load_and_extend_table("assets/actors.toml").await;
        Infos {}
    }
}
