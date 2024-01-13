use cursive::align::HAlign;
use cursive::views::{Dialog, EditView, LinearLayout, NamedView, SelectView, TextView};
use cursive::Cursive;
use cursive::{traits::*, CursiveRunnable};

use crate::local_repository::LocalRepository;

pub struct UI {
    pub branch_names: Vec<String>,
    pub repository: LocalRepository,
    siv: CursiveRunnable,
}

impl UI {
    pub fn from_branch_names(branch_names: Vec<String>) -> Self {
        Self {
            branch_names,
            ..Default::default()
        }
    }

    pub fn from_repository(repository: LocalRepository) -> Self {
        Self {
            repository,
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
        select.add_all_str(self.repository.branch_names());

        // Sets the callback for when "Enter" is pressed.
        select.set_on_submit(|siv, branch_name| {
            match LocalRepository::default().checkout(branch_name) {
                Ok(_) => {
                    show_message_and_quit(siv, "Todo OK!!");
                }
                Err(e) => show_message_and_quit(siv, e.message()),
            }
        });
        select.with_name("branches")
    }

    fn edit_view(&self) -> NamedView<EditView> {
        let branch_names = self.repository.branch_names();
        EditView::new()
            // update results every time the query changes
            .on_edit(move |siv, text, _cursor| {
                let found_branch_names: Vec<String> = branch_names
                    .iter()
                    .filter(|&name| {
                        name.to_lowercase()
                            .contains(&String::from(text).to_lowercase())
                    })
                    .cloned()
                    .collect();

                siv.call_on_name("branches", |v: &mut SelectView| {
                    v.clear();
                    v.add_all_str(found_branch_names);
                });
            })
            // submit the focused (first) item of the matches
            //.on_submit()
            .with_name("query")
    }
}

impl Default for UI {
    fn default() -> Self {
        Self {
            branch_names: vec![],
            repository: LocalRepository::default(),
            siv: cursive::default(),
        }
    }
}
fn show_message_and_quit(siv: &mut Cursive, message: &str) {
    siv.pop_layer();
    siv.add_layer(Dialog::around(TextView::new(message)).button("Quit", |s| s.quit()));
}
