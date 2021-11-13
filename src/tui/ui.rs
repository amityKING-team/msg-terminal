use {
  cursive::CursiveRunnable,
  std::{
    fs::{read_to_string, File},
    //result,
    path::Path,
    //slice,
    str,
  },
  toml::Value,
  config::Config,
};

#[path = "../msg_config/mod.rs"]
pub mod impl_config;
//use impl_config;
//pub mod TemrinalConfig;
//use impl_config::TemrinalConfig;

// Does `pub`?
pub struct TerminalUI {
  siv: CursiveRunnable,
  config: Value,
}

impl TerminalUI {
  fn have_config_file() -> bool {
    let path = Path::new(impl_config::TerminalConfig::config_path());
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
