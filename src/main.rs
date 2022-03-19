mod configuration;
mod tui;

use std::{
  collections::HashMap,
  fmt::{Debug, Display},
  fs::read_to_string,
};

use serde::Deserialize;
use toml::Value;
use tui::ui::TerminalUI;
//use cursive::align::{Align, HAlign};
//use cursive::traits::*;
// use cursive::views::{Dialog, DummyView, LinearLayout, TextView};

// use termion::terminal_size;

// use cursive::event::Key;
// use cursive::views::*;

fn main() {
  let string_file = match read_to_string("src/assets/config.toml") {
    Err(why) => panic!("Can't open configuration file: {} \t :(", why),
    Ok(string) => string,
  };

  let value_config: HashMap<String, toml::Value> = toml::from_str(&string_file).unwrap();
  let mut config: HashMap<String, Box<dyn Display + 'static>> = HashMap::new();
  for (key, value) in value_config {
    let config_key = key.clone();
    // TODO: Add datetime, etc.
    let config_value: Box<dyn Display + 'static> = match value {
      toml::Value::String(val) => Box::new(val),
      toml::Value::Integer(num) => Box::new(num),
      toml::Value::Float(float) => Box::new(float),
      toml::Value::Boolean(boolean) => Box::new(boolean),
      _ => Box::new(1),
    };
    // Represents a TOML datetime
    // Datetime(Datetime),
    // Represents a TOML array
    // Array(Array),
    // Represents a TOML table
    // Table(Table),
    config.insert(config_key, config_value);
  }
  for (key, value) in &config {
    println!("{}: {}", key, value);
  }
  assert_eq!("toml", *config["name"]);
  //let temrinal = TerminalUI::default();
  /*
  let mut cursive = cursive::default();

  let temrinal_size = termion::terminal_size().unwrap();

  let mut friends_list = SelectView::new().h_align(HAlign::Center);
  friends_list.add_item("1", 1);
  friends_list.add_item("2", 2);
  friends_list.add_item("3", 3);

  let friends_dialog = Panel::new(friends_list);
  let dialog_and_input_layer = LinearLayout::vertical()
  .child(TextView::new("Top of the page"))
  .child(TextArea::new().fixed_height((temrinal_size.1 - 4) as usize))
  //.fixed_size((30, 10)))
  .child(Button::new("Ok", |s| s.quit()));

  let fullscreen_layer = LinearLayout::horizontal()
  .child(friends_dialog)
  .child(dialog_and_input_layer);
  cursive.add_fullscreen_layer(Dialog::around(fullscreen_layer));


  cursive.run();*/
}

/*fn main() {

  let mut siv = cursive::default();

  siv.load_theme_file("src/assets/style.toml").unwrap();

  // We can quit by pressing q
  siv.add_global_callback('q', |event| event.quit());

  tui::create_default_theme(&mut siv);
  tui::create_default_windows(&mut siv);

  siv.run();

}*/
