use cursive::align::HAlign;
use cursive::event::EventResult;
use cursive::traits::*;
use cursive::views::{Dialog, OnEventView, SelectView, TextView};
use cursive::Cursive;

pub struct UI {
    pub branch_names: Vec<String>,
}

impl UI {
    pub fn start(&self) {
        let mut select: SelectView = SelectView::new()
            // Center the text horizontally
            .h_align(HAlign::Center)
            // Use keyboard to jump to the pressed letters
            .autojump();

        select.add_all_str(self.branch_names.clone());

        // Sets the callback for when "Enter" is pressed.
        select.set_on_submit(show_next_window);

        // Let's override the `j` and `k` keys for navigation
        let select = OnEventView::new(select)
            .on_pre_event_inner('k', |s, _| {
                let cb = s.select_up(1);
                Some(EventResult::Consumed(Some(cb)))
            })
            .on_pre_event_inner('j', |s, _| {
                let cb = s.select_down(1);
                Some(EventResult::Consumed(Some(cb)))
            });

        let mut siv = cursive::default();

        // Let's add a ResizedView to keep the list at a reasonable size
        // (it can scroll anyway).
        siv.add_layer(
            Dialog::around(select.scrollable().fixed_size((20, 10))).title("Select your branch..."),
        );

        siv.run();
    }
}

// Let's put the callback in a separate function to keep it clean,
// but it's not required.
fn show_next_window(siv: &mut Cursive, city: &str) {
    siv.pop_layer();
    let text = format!("let's checkout out {city}");
    siv.add_layer(Dialog::around(TextView::new(text)).button("Quit", |s| s.quit()));
}
