use std::{collections::HashMap, rc::Rc};

use glam::Vec2;
use macroquad::{file::load_file, texture::Texture2D};
use toml::Table;


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
async fn load_and_extend_table(path:&str) -> Table {
    let actors = String::from_utf8(load_file(path).await.unwrap()).unwrap();
    let table: toml::Table = toml::from_str(&actors).unwrap();
    extend_table(table)
}

pub struct ImageInfo2 {
    pub name: String,
    pub path: String,
    pub texture: Texture2D,
}

#[derive(Clone)]
pub struct FrameIndex2 {
    pub image: Rc<ImageInfo2>,
    pub frame: u16,
}


#[derive(Clone, Default)]
pub struct WeaponInfo2 {
    pub rate_of_fire: f32,
    pub name: String,
    pub frames: Vec<FrameIndex2>,
    pub damage: [f32; 2],
    pub mount_offset: f32,
    pub muzzle_offset: f32,
    pub spread:f32
}

#[derive(Clone)]
pub struct ActorInfo2 {
    pub name: String,
    pub frames: Vec<FrameIndex2>,
    pub locomotion_frames: Vec<FrameIndex2>,
    pub dead_frames: Vec<FrameIndex2>,
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
    
}
impl Infos {
    pub async fn init() -> Self {
        let actors = load_and_extend_table("assets/actors.toml").await;
        let weapons = load_and_extend_table("assets/weapons.toml").await;
        Infos {}
    }
}
