use std::fs::File;
use std::io::Read;
use std::path::Path;

use toml;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub token: String,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(name: P) -> ::Result<Self> {
        let mut file = File::open(name)?;
        let mut toml_doc = String::new();
        file.read_to_string(&mut toml_doc)?;
        Ok(toml::from_str(&toml_doc)?)
    }
}
