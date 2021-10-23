use cursive::{
  align::HAlign,
  event::{EventResult, Key},
  traits::With,
  view::{scroll::Scroller, Scrollable},
  views::{Dialog, OnEventView, Panel, TextView},
};

pub fn create_default_windows(siv: &mut cursive::CursiveRunnable) {
  // Read some long text from a file.
  let content = include_str!("../assets/lorem.txt");
  
  siv.add_fullscreen_layer(
    Dialog::around(Panel::new(
      TextView::new(content)
        .scrollable()
        .wrap_with(OnEventView::new)
        .on_pre_event_inner(Key::PageUp, |v, _| {
          let scroller = v.get_scroller_mut();
          if scroller.can_scroll_up() {
            scroller.scroll_up(scroller.last_outer_size().y.saturating_sub(1));
          }
          Some(EventResult::Consumed(None))
        })
        .on_pre_event_inner(Key::PageDown, |v, _| {
          let scroller = v.get_scroller_mut();
          if scroller.can_scroll_down() {
            scroller.scroll_down(scroller.last_outer_size().y.saturating_sub(1));
          }
          Some(EventResult::Consumed(None))
        }),
    ))
    .title("Unicode and wide-character support")
    // This is the alignment for the button
    .h_align(HAlign::Center)
    .button("Quit", |s| s.quit()),
  );
  // Show a popup on top of the view.
  siv.add_layer(Dialog::info(
    "Try resizing the terminal!\n(Press 'q' to \
             quit when you're done.)",
  ));
}

pub fn create_default_theme(siv: &mut cursive::CursiveRunnable) {
  // You can load a theme from a file at runtime for fast development.
  siv.load_theme_file("src/assets/style.toml").unwrap();

  // Or you can directly load it from a string for easy deployment.
  siv.load_toml(include_str!("../assets/style.toml")).unwrap();

}