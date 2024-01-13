use cursive::align::HAlign;
use cursive::event::EventResult;
use cursive::views::{Dialog, OnEventView, SelectView, TextView};
use cursive::Cursive;
use cursive::{traits::*, CursiveRunnable};

pub struct UI {
    pub branch_names: Vec<String>,
    siv: CursiveRunnable,
}

impl UI {
    pub fn from_branch_names(branch_names: Vec<String>) -> Self {
        Self {
            branch_names,
            ..Default::default()
        }
    }
    pub fn start(&mut self) {
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

        // Let's add a ResizedView to keep the list at a reasonable size
        // (it can scroll anyway).
        self.siv.add_layer(
            Dialog::around(select.scrollable().fixed_size((20, 10))).title("Select your branch..."),
        );

        self.siv.run();
    }
}

impl Default for UI {
    fn default() -> Self {
        Self {
            branch_names: vec![],
            siv: cursive::default(),
        }
    }
}

// Let's put the callback in a separate function to keep it clean,
// but it's not required.
fn show_next_window(siv: &mut Cursive, city: &str) {
    siv.pop_layer();
    let text = format!("let's checkout out {city}");
    siv.add_layer(Dialog::around(TextView::new(text)).button("Quit", |s| s.quit()));
}
