use std::{env::consts::FAMILY, fs, path::Path};

use serde_derive::{Deserialize, Serialize};
use toml::from_str;

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
  theme: &'static str,
  lang: &'static str,
}

// impl<'l> Configuration {
//   fn parse(read_str: &'l str) -> Result<Configuration, toml::de::Error> {
//     let config = toml::from_str(&read_str);

//     config
//   }
// }

impl<'l> Configuration {
  pub fn open() -> std::io::Result<Configuration> {
    let path_string = match Configuration::path() {
      Some(x) => x,
      None => Err("Can't define OS family"),
    };
    let read_string = fs::read(path_string);

    Ok(toml::from_str(*read_string)?)
  }

  pub fn path<'a>() -> Option<&'a Path> {
    // TODO: Maybe use OS than FAMILY
    return match FAMILY {
      "unix" => Some(Path::new("src/assets/config.toml")),
      "windows" => Some(Path::new("src/assets/config.toml")),
      _ => None,
    };
  }
}

impl Default for Configuration {
  fn default() -> Self {
    let default_config = Configuration {
      theme: "default",
      lang: "en",
    };

    default_config
  }
}
