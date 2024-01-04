use macroquad::file::load_file;
use toml::Table;

pub struct Metadata2 {}

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

impl Metadata2 {
    pub async fn init() -> Self {
        let actors = String::from_utf8(load_file("assets/actors.toml").await.unwrap()).unwrap();
        let table: toml::Table = toml::from_str(&actors).unwrap();
        let table = extend_table(table);
       
        Metadata2 {}
    }
}
