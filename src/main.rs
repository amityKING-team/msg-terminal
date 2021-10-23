mod tui;

fn main() {
  let mut siv = cursive::default();

  //siv.load_theme_file("src/assets/style.toml").unwrap();

  // We can quit by pressing q
  siv.add_global_callback('q', |event| event.quit());

  //tui::create_default_theme(&mut siv);
  tui::create_default_windows(&mut siv);

  siv.run();
}
