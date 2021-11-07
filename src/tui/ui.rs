use {
  cursive::CursiveRunnable,
  std::{
    env::consts::FAMILY,
    fs::{read_to_string, File},
    //result,
    path::Path,
    //slice,
    str,
  },
  toml::Value,
};

// Does `pub`?
pub struct TerminalUI {
  siv: CursiveRunnable,
  config: Value,
}

impl TerminalUI {
  // TODO: Add 
  fn define_os() -> Option<String> {
    todo!();
    //Some("unix".to_string())
  }

  pub fn config_path() -> &'static str {
    // TODO: Maybe use OS than FAMILY
    let os: &str = FAMILY;
    return match os {
      "unix" => "src/assets/config.toml",
      // TODO: Look universal string for all platforms
      "windows" => "src/assets/config.toml",
      _ => "unknown",
    };
  }

  fn have_config_file() -> bool {
    let path = Path::new(TerminalUI::config_path());
    return match File::open(path) {
      Ok(_) => true,
      Err(_) => false,
    };
  }

  pub fn config() -> Result<toml::Value, &'static str> {
    if TerminalUI::have_config_file() {
      //let file = File::open(TerminalUI::config_path()).unwrap();
      let data = read_to_string(TerminalUI::config_path()).unwrap();
      //println!("{}", data);
      let toml_config = data.parse::<Value>().unwrap();
      //.unwrap();
      //toml::from_str(&data).unwrap();
      //println!("{:?}", json);
      return Ok(toml_config);

    }
    Err("No config file")
  }
}

impl Default for TerminalUI {
  fn default() -> Self {
    //let os = TerminalUI::define_os().unwrap();

    let config = TerminalUI::config();

    let mut siv = cursive::default();

    let temrinal = TerminalUI {
      siv: siv,
      config: config.unwrap(),
    };
    temrinal
  }
}
