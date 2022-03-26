use std::{collections::HashMap, env::consts::FAMILY, fmt::Display, fs::read_to_string};

use serde_derive::{Deserialize, Serialize};
use toml::from_str;

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
  theme: &'static str,
  lang: &'static str,
  // config: HashMap<String, Box<dyn Display + 'static>>,
}

impl Configuration {
  fn read() -> Result<String, std::io::Error> {
    let read_string = read_to_string(Configuration::path());
    return read_string;
  }
  fn parse<'a>(read_string: &'a Result<String, std::io::Error>) -> Option<Configuration> {
    let config: Option<Configuration> = match read_string {
      Ok(str_ok) => toml::from_str(&str_ok).ok(),
      _ => None,
    };
    return config;
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

impl Configuration {
  pub fn new() -> Configuration {
    let config_string = Configuration::read();
    let config = match Configuration::parse(config_string) {
      Some(config) => config,
      None => Configuration::default(),
    };

    return config;
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
