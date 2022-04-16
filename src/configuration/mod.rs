use std::{env::consts::FAMILY, fs::read_to_string};

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
    let read_string: String = read_to_string(Configuration::path())?;

    Ok(toml::from_str(&read_string)?)
  }

  pub fn path() -> &'static str {
    // TODO: Maybe use OS than FAMILY
    return match FAMILY {
      "unix" => &"src/assets/config.toml",
      "windows" => &"src/assets/config.toml",
      _ => "unknown",
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
