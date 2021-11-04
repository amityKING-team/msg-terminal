mod tui;

use cursive::align::{HAlign, Align};
use cursive::traits::*;
use cursive::views::{Dialog, DummyView, LinearLayout, TextView};

use termion::terminal_size;

// This example uses a LinearLayout to stick multiple views next to each other.

use cursive::event::Key;
use cursive::views::*;

fn main() {
    let mut cursive = cursive::default();

    //cursive.set_fps(30);

    let temrinal_size = termion::terminal_size().unwrap();

    // Create a view tree with a TextArea for input, and a
    // TextView for output.
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
          /*SizeConstraint::Fixed(10),
                            SizeConstraint::Fixed(10),
                            Panel::new(TextArea::new()
                                .content("")
                                .with_name("input"))))
        .child(BoxView::new(SizeConstraint::Fixed(10),
                            SizeConstraint::Fixed(10),
                            Panel::new(TextView::new("")
                                .with_id("output")))
                              */
    // cursive.add_global_callback(Key::Esc, |c| {
    //     // When the user presses Escape, update the output view
    //     // with the contents of the input view.
    //     let input = c.find_id::<TextArea>("input").unwrap();
    //     let mut output = c.find_id::<TextView>("output").unwrap();
    //     output.set_content(input.get_content());
    // });

    cursive.run();
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

