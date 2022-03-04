use std::str;

use cursive::CursiveRunnable;
use toml::Value;

use crate::configuration::msg_configuration as msg_config;

// Does `pub`?
pub struct TerminalUI {
  siv: CursiveRunnable,
  config: msg_config::Configuration,
}

impl TerminalUI {
  /*pub fn config() -> Result<toml::Value, &'static str> {
    if TerminalUI::check_config_file() {
      //let file = File::open(TerminalUI::config_path()).unwrap();
      let data = read_to_string(msg_config::Configuration::config_path()).unwrap();
      //println!("{}", data);
      let toml_config = data.parse::<Value>().unwrap();
      //.unwrap();
      //toml::from_str(&data).unwrap();
      //println!("{:?}", json);
      return Ok(toml_config);

    }
    Err("No config file")
  }*/
}

/*impl Default for TerminalUI {
  fn default() -> Self {
    //let os = TerminalUI::define_os().unwrap();

    let config = TerminalUI::config();
    let config = match config {
      Ok(config) => config,
      Err(config) =>
    };

    let mut siv = cursive::default();

    let temrinal = TerminalUI {
      siv: siv,
      config: config,
    };
    temrinal
  }
}*/
