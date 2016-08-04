extern crate cursive;

use cursive::prelude::*;

fn main() {
    let mut siv = Cursive::new();

    siv.add_layer(Dialog::empty()
        .title("Describe your issue")
        .padding((1, 1, 1, 0))
        .content(TextArea::new()
            .with_id("text")
            .fixed_size((30, 5)))
        .button("Ok", Cursive::quit));

    siv.run();
}
