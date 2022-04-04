use std::{env::consts::FAMILY, fs::read_to_string};

use serde_derive::{Deserialize, Serialize};
use toml::from_str;

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
  theme: &'static str,
  lang: &'static str,
}

impl<'l> Configuration {
  fn parse(read_str: &'l Result<String, std::io::Error>) -> Result<Configuration, toml::de::Error> {
    let read_string = read_str.unwrap();
    let config = toml::from_str(&read_string);

    config
  }
}

impl Configuration {
  pub fn new() -> Configuration {
    let read_string = match read_to_string(Configuration::path()) {
      Ok(result) => result,
      Err(why) => println!("Can't read file: {}", why)
    };

    let config = match Configuration::parse(&read_string) {
      Ok(result) => result,
      Err(why) => {
        print!("Can't parse file, default config");
        Configuration::default()
      }
    };
    /*match Configuration::parse(config_string) {
      Some(config) => config,
      None => Configuration::default(),
    };*/

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

impl Default for Configuration {
  fn default() -> Self {
    let default_config = Configuration {
      theme: "default",
      lang: "en",
    };

    default_config
  }
}
