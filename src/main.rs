use cursive::With;

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    siv.add_global_callback('q', cursive::Cursive::quit);

    // siv.add_layer(cursive::views::TextView::new(
    //     "Hello, world!\n\
    //      Press 'q' to quit the application.",
    // ));
    siv.add_layer(
        cursive::views::Dialog::around(cursive::views::TextView::new("Hello Dialog!"))
            .title("Cursive")
            .button("Foo", |_s| ())
            .button("Quit", |s| s.quit())
            .wrap_with(cursive::views::CircularFocus::new)
            .wrap_tab(),
    );

    siv.run();
}
