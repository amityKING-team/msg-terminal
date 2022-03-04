#[path = "../src/tui/mod.rs"]
pub mod tui;

use std::{fs::File, path::Path};

use tui::ui::TerminalUI;

#[cfg(test)]
mod terminaui_test {
  use super::*;

  #[test]
  #[cfg(target_os = "linux")]
  fn config_path_test() {
    let test_path = "src/assets/config.toml";
    assert_eq!(test_path, TerminalUI::config_path())
  }

  #[test]
  #[cfg(target_os = "windows")]
  // TODO: Add windows path
  fn config_path_test() {
    let test_path = "src/assets/config.toml";
    assert_eq!(test_path, TerminalUI::config_path())
  }

  #[test]
  fn have_config_file_test() {
    let path = Path::new(TerminalUI::config_path());
    let optional_path = match File::open(path) {
      Ok(_) => true,
      Err(_) => false,
    };
    assert!(optional_path)
  }

  #[test]
  fn config_test() {
    let toml_config = TerminalUI::config().unwrap();
    //let toml_config = toml_result.unwrap();
    println!("{}", toml_config["package"]["name"]);
    assert_eq!(Some("toml"), toml_config["package"]["name"].as_str());
  }
}
