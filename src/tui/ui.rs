use {
  cursive::Cursive,
  std::{
    env::consts::FAMILY,
    fs::File,
    //result,
    path::Path,
    str,
    slice,
  },
  toml::Value,
};

// Does `pub`?
pub struct TerminalUI {
  siv: cursive::CursiveRunnable,
  config: toml::Value,
}

impl TerminalUI {
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

  fn have_config_file() -> bool {
    //let directory = fs::read_dir("src/assets").unwrap();
    let path = Path::new(TerminalUI::config_path());
    let data: Result<&str, &'static str>;
    return match File::open(path) {
      Ok(_) => true,
      Err(_) => false,
    };
    //if(data.is_ok()) {
    //} else {
    //  return None
    //}
  }

  pub fn config() -> Result<toml::Value, &'static str> {
    if TerminalUI::have_config_file() {
      let json: toml::Value = toml::from_str(TerminalUI::config_path()).unwrap();
      return Ok(json);
    }
    Err("No config file")
  }
}

impl Default for TerminalUI {
  fn default() -> Self {
    let os = TerminalUI::define_os().unwrap();

    let config = TerminalUI::config();

    let mut siv = cursive::default();

    let temrinal = TerminalUI {
      siv: siv, 
      config: config.unwrap(),
    };
    temrinal
  }
}
