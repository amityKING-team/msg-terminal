use {
  cursive::{
    Cursive
  },
  serde_json::{
    json,
    Result,
    Value
  },
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
    /*let os = cfg_match! {

    };
    Some(os.to_string())*/
    Some("unix".to_string())
  }

  fn find_config_file() -> Result<String>{
    let directory = fs::read_dir("src/assets").unwrap();
    for item in directory {
      if item.unwrap().path() == "config.toml" {
        let data = fs::read_to_string(item).expect("Unable to read file");
        let json = 
      }
    }
    Ok("1".to_string())
  }

  pub fn config() -> Result<serde_json::Value> {
    if()
    Ok()
  }
}

impl Default for TerminalUI {
  fn default() -> Self {

    let os = TerminalUI::define_os().unwrap();

    let config = TerminalUI::config();

    let mut siv = cursive::default();

    let temrinal = TerminalUI{};
    temrinal
  }




  // search_directory() -> Result<> {

  // }
}