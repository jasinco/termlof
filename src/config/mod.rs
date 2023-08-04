use toml::Table;
use std::{io::Read,fs};
use serde::Deserialize;


#[derive(Deserialize)]
pub struct Config{
    lofi: Table,
    music: Table,
}

impl Config{
    pub fn lofilist(&self) -> Vec<String>{
        return self.lofi.keys().into_iter().map(|f|{f.into()}).collect::<Vec<String>>();
    }
    pub fn musiclist(&self) -> Vec<String>{
        return self.music.keys().into_iter().map(|f|{f.into()}).collect::<Vec<String>>();
    }
}

pub fn parse_default() -> std::io::Result<Config> {
    let mut file = fs::File::open("./termlof.toml")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let parsed:Config = toml::from_str(&content).unwrap();
    Ok(parsed)
}
