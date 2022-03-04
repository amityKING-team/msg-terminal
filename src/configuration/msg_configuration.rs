use std::{collections::HashMap, env::consts::FAMILY, fmt::Display, fs::read_to_string};

use toml::Value;

pub struct Configuration {
  config: HashMap<String, Box<dyn Display + 'static>>,
}

impl Configuration {
  fn read() -> HashMap<String, Box<dyn Display + 'static>> {
    let string_file = match read_to_string(Configuration::path()) {
      Err(why) => panic!("Can't open configuration file: {} \t :(", why),
      Ok(string) => string,
    };

    let value_config: HashMap<String, toml::Value> = toml::from_str(&string_file).unwrap();
    let mut config: HashMap<String, Box<dyn Display + 'static>> = HashMap::new();
    for (key, value) in value_config {
      let config_key = key.clone();
      // TODO: Add datetime, etc.
      let config_value: Box<dyn Display + 'static> = match value {
        toml::Value::String(val) => Box::new(val),
        toml::Value::Integer(num) => Box::new(num),
        toml::Value::Float(float) => Box::new(float),
        toml::Value::Boolean(boolean) => Box::new(boolean),
        _ => Box::new(1),
      };
      // Represents a TOML datetime
      // Datetime(Datetime),
      // Represents a TOML array
      // Array(Array),
      // Represents a TOML table
      // Table(Table),
      config.insert(config_key, config_value);
    }
    return config
  }
}

impl Configuration {
  pub fn new() -> Configuration {
    //let configObject =
    let config = Configuration { config: Configuration::read() };

    return config
  }

  pub fn path() -> &'static str {
    // TODO: Maybe use OS than FAMILY
    return match FAMILY {
      "unix" => &"src/assets/config.toml",
      "windows" => &"src/assets/config.toml",
      _ => "unknown",
    }
  }
}
