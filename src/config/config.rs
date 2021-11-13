// use

pub struct Config {
  
}

impl Config {
  fn define_os() -> Option<String> {
    /*let os = cfg_match! {

    };
    Some(os.to_string())*/
    Some("unix".to_string())
  }

  fn config_path() -> &'static str {
    let os: String = FAMILY.to_string();
    return match os {
      "linux" => "src/assets/config.toml",
      "windows"=> "src/assets/config.toml",
    };
  }

}