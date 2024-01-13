use cursive::align::HAlign;
use cursive::views::{Dialog, EditView, LinearLayout, NamedView, SelectView, TextView};
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
        let select_view = self.select_view();
        let edit_view = self.edit_view();

        self.siv.add_layer(
            Dialog::around(
                LinearLayout::vertical()
                    .child(edit_view)
                    .child(select_view.scrollable().fixed_size((20, 10))),
            )
            .title("Select your branch..."),
        );

        self.siv.run();
    }

    fn select_view(&self) -> NamedView<SelectView> {
        let mut select: SelectView = SelectView::new()
            // Center the text horizontally
            .h_align(HAlign::Center)
            // Use keyboard to jump to the pressed letters
            .autojump();

        select.add_all_str(self.branch_names.clone());

        // Sets the callback for when "Enter" is pressed.
        select.set_on_submit(show_next_window);
        select.with_name("branches")
    }

    fn edit_view(&self) -> NamedView<EditView> {
        EditView::new()
            // update results every time the query changes
            //.on_edit(on_edit)
            // submit the focused (first) item of the matches
            //.on_submit(on_submit)
            .with_name("query")
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
