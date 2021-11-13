use {
  std::{
    env::consts::FAMILY
  },
  config::Config,
};

pub trait TemrinalConfig {
  fn config_path() -> &'static str;
  fn define_os() -> Option<String>;
}

impl TemrinalConfig for Config {
  fn config_path() -> &'static str {
    // TODO: Maybe use OS than FAMILY
    let os: &str = FAMILY;
    return match os {
      "unix" => "src/assets/config.toml",
      // TODO: Look universal string for all platforms
      "windows" => "src/assets/config.toml",
      _ => "unknown",
    };
  }

  fn define_os() -> Option<String> {
    /*let os = cfg_match! {

    };
    Some(os.to_string())*/
    Some("unix".to_string())
  }
}
  // fn config_path() -> &'static str {
  //   let os: String = FAMILY.to_string();
  //   return match os {
  //     "linux" => "src/assets/config.toml",
  //     "windows"=> "src/assets/config.toml",
  //   };