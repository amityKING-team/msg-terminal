use serde_derive::{Deserialize, Serialize};
use std::{env::consts::FAMILY, fs, path::Path};
use toml::from_str;

struct CustomError(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
  theme: String,
  lang: String,
}

impl Configuration {
  pub fn path<'a>() -> Option<&'a str> {
    // TODO: Maybe use OS than FAMILY
    return match FAMILY {
      "unix" => Some(&"src/assets/Config.toml"),
      "windows" => Some(&"src/assets/Config.toml"),
      _ => None,
    };
  }

  pub fn open() -> Result<String, String> {
    let path_string = match Configuration::path() {
      Some(x) => Ok(x),
      None => Err("Can't define OS"),
    };
    let config_string = match path_string {
      Ok(result) => match fs::read_to_string(result) {
        Ok(string) => string,
        Err(err) => err.to_string(),
      },
      Err(err) => format!("{} \n {}", err, &"Can't open file"),
    };
    Ok(config_string)
  }

  pub fn parse(config_string: &str) -> Result<Configuration, String> {
    let config: Result<Configuration, String> = match toml::from_str(&config_string[..]) {
      Ok(result) => result,
      Err(err) => Err(format!("Can't parse config file: {}", err)),
    };
    config
  }
}

/*impl Default for Configuration {
  fn default() -> Self {
    let default_config = Configuration {
      theme: "default",
      lang: "en",
    };

    default_config
  }
}*/
