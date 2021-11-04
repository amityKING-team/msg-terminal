use {
  cursive::{
    Cursive
  },
  serde_json::json,
  std::fs
};
//use serde_json::json;

// Does `pub`?
pub struct TerminalUI {
  siv: cursive::Cursive,
  config: serde_json::Value 
}

impl TerminalUI {
  pub fn define_os() -> Option<String> {
    let os = cfg_match! {

    };
    Some(os.to_string())
  }
}

impl Default for TerminalUI {
  fn default() -> Self {

    let os = TerminalUI::define_os();

    let mut siv = cursive::default();

    let temrinal = TerminalUI{};
    temrinal
  }



  fn find_config_file() -> Result<String>{
    let direcory = fs::read_dir("../assets"); 
  }

  search_directory() -> Result<> {

  }
}