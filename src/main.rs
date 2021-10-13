//use cursive::{
//    views::{Dialog, TextView},
//    Cursive};
use cursive::views::{Dialog, TextView};

fn main() {
    let mut siv = cursive::default();
                        //Cursive::new();

    // You can load a theme from a file at runtime for fast development.
    siv.load_theme_file("src/assets/style.toml").unwrap();

    // Or you can directly load it from a string for easy deployment.
    siv.load_toml(include_str!("assets/style.toml")).unwrap();

    let mut dialog1 = Dialog::around(TextView::new(
            "This application uses a \
             custom theme!",
        ))
        .title("Themed dialog")
        .button("esc", |s| s.quit());
    
    //dialog1.buttons().set_label_raw("Quit");
    
    let but1 = dialog1.buttons_mut().next().unwrap();
    but1.set_label_raw("[Quit]");

    siv.add_layer(dialog1);
    
    siv.run();
}
