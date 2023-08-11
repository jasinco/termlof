use toml::Table;
use std::{io::{Read, self},fs, path};
use dirs;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config{
    lofi: Table,
    music: Table,
}

impl Config{
    pub fn lofilist(&self) -> Vec<&str>{
        return self.lofi.keys().into_iter().map(|f|{f.as_str()}).collect::<Vec<&str>>();
    }
    pub fn musiclist(&self) -> Vec<&str>{
        return self.music.keys().into_iter().map(|f|{f.as_str()}).collect::<Vec<&str>>();
    }
    pub fn get_val(&self, key:&str) -> String{
        if let Some(st) = self.lofi.get(key){
            return st.as_str().unwrap().to_string()
        }else if let Some(stm) = self.music.get(key){
            return stm.as_str().unwrap().to_string()
        }else{
            return "".to_string();
        }
    }
}

pub fn parse_default() -> io::Result<Config> {
    let config = dirs::config_dir().unwrap().join("termlof.toml");
    if let Ok(mut file) = fs::File::open(config){
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let parsed:Config = toml::from_str(&content).unwrap();
        Ok(parsed)
    }else{
        Err(io::Error::new(io::ErrorKind::NotFound, "Didn't found termlof.toml"))
    }
}
