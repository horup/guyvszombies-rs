use std::collections::HashMap;

use macroquad::file::load_file;
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

async fn load_and_extend_table(path:&str) -> Table {
    let actors = String::from_utf8(load_file(path).await.unwrap()).unwrap();
    let table: toml::Table = toml::from_str(&actors).unwrap();
    extend_table(table)
}

type InfoIndex = u16;

pub trait Info {
    fn name(&self) -> &str;
    fn index(&self) -> InfoIndex;
}

pub struct InfoCollection<T> {
    inner: Vec<T>,
    name_to_index: HashMap<String, InfoIndex>,
}

impl<T:Info> InfoCollection<T> {
    
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
