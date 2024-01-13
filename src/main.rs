use crate::local_repository::LocalRepository;
use crate::ui::UI;

pub mod local_repository;
pub mod ui;

fn main() {
    let repo = LocalRepository::new();
    let mut app = UI::from_repository(repo);
    app.start();
}
