use crate::local_repository::LocalRepository;
use crate::ui::UI;

pub mod local_repository;
pub mod ui;
// We'll use a SelectView here.
//
// A SelectView is a scrollable list of items, from which the user can select
// one.

fn main() {
    let repo = LocalRepository::new();
    let mut app = UI::from_branch_names(repo.branch_names());
    app.start();
}
