use {
  std::{
    env::consts::FAMILY,
    fs::{read_to_string, File},
    path::Path,
  },
  std::collections::HashMap;
};

pub struct Configuration {
  config: HashMap, 
  path: &'static str,
}

impl Configuration {

  pub fn new() -> Configuration {
    let configObject = 
    let config = Configuration {
      config: read(),
      path: path(),
    };

    return config
  }

  pub fn read() -> config::Config {
    let read_config = config::Config::builder()
        .add_source();

    return read_config
  }

  pub fn path() -> &str {
    // TODO: Maybe use OS than FAMILY
    return match Configuration::define_os_family() {
      "unix" => &"src/assets/config.toml",
      "windows" => &"src/assets/config.toml",
      _ => "unknown",
    }
  }

  fn define_os_family() -> &str {
    //Some(os.to_string())
    //Some("unix".to_string())
    return match FAMILY {
      "unix" => &"unix",
      "windows" => &"windows",
      _ => &"unknown",
    }
  }

  fn check_file() -> bool {
    let path = Path::new(msg_config::Configuration::config_path());
    return match File::open(path) {
      Ok(_) => true,
      Err(_) => false,
    }
}