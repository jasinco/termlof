use std::process;
use std::{result, error, io};

use serde_json::Value;



pub fn get_audio_url(url:&str) -> result::Result<String, Box<dyn error::Error>>{
    let output = process::Command::new("./yt-dlp").args([url, "-x","--skip-download", "-j"]).output()?;
    let stro = String::from_utf8(output.stdout)?;
    let err = String::from_utf8(output.stderr)?;
    if err == ""{
        let val:Value = serde_json::from_str(&stro)?;
        // TODO: The error message of ytdlp
        let url = val.get("url").unwrap().as_str().unwrap().to_string();
        Ok(url)
    }else{
        Err(io::Error::new(io::ErrorKind::InvalidInput, "Wrong url").into())
    }

}