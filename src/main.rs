mod configuration;
mod tui;

use std::{
  collections::HashMap,
  fmt::{Debug, Display},
  fs::read_to_string,
};

use toml::Value;
use tui::ui::TerminalUI;
//use cursive::align::{Align, HAlign};
//use cursive::traits::*;
// use cursive::views::{Dialog, DummyView, LinearLayout, TextView};

// use termion::terminal_size;

// use cursive::event::Key;
// use cursive::views::*;

fn main() {
  let config = match configuration::Configuration::open() {
    Ok(result) => result,
    Err(_) => configuration::Configuration::default(),
  };
  println!("{:?}", &config);

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
