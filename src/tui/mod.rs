use cursive::{
  //align::HAlign,
  event::{Event, EventResult, Key},
  traits::*,
  //view::{scroll::Scroller, Scrollable},
  views::{Dialog, OnEventView, Panel, TextView, EditView, TextArea},
  Cursive
};

mod ui;

pub fn create_default_windows(siv: &mut cursive::CursiveRunnable) {
  //let content = include_str!("../assets/lorem.txt");  

      siv.add_layer(
          Dialog::new()
              .title("Describe your issue")
              .padding_lrtb(1, 1, 1, 0)
              .content(TextArea::new().with_name("text"))
              .button("Ok", Cursive::quit),
      );
  
      // We'll add a find feature!
      siv.add_layer(Dialog::info("Hint: press Ctrl-F to find in text!"));
  
      siv.add_global_callback(Event::CtrlChar('f'), |s| {
          // When Ctrl-F is pressed, show the Find popup.
          // Pressing the Escape key will discard it.
          s.add_layer(
              OnEventView::new(
                  Dialog::new()
                      .title("Find")
                      .content(
                          EditView::new()
                              .on_submit(find)
                              .with_name("edit")
                              .min_width(10),
                      )
                      .button("Ok", |s| {
                          let text = s
                              .call_on_name("edit", |view: &mut EditView| {
                                  view.get_content()
                              })
                              .unwrap();
                          find(s, &text);
                      })
                      .dismiss_button("Cancel"),
              )
              .on_event(Event::Key(Key::Esc), |s| {
                  s.pop_layer();
              }),
          )
      });

  /*siv.add_fullscreen_layer(
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
  );*/

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

  fn find(siv: &mut Cursive, text: &str) {
      // First, remove the find popup
      siv.pop_layer();
  
      let res = siv.call_on_name("text", |v: &mut TextArea| {
          // Find the given text from the text area content
          // Possible improvement: search after the current cursor.
          if let Some(i) = v.get_content().find(text) {
              // If we found it, move the cursor
              v.set_cursor(i);
              Ok(())
          } else {
              // Otherwise, return an error so we can show a warning.
              Err(())
          }
      });
  
      if let Some(Err(())) = res {
          // If we didn't find anything, tell the user!
          siv.add_layer(Dialog::info(format!("`{}` not found", text)));
      }
  }
