use cursive::align::HAlign;
use cursive::event::EventResult;
use cursive::traits::*;
use cursive::views::{Dialog, OnEventView, SelectView, TextView};
use cursive::Cursive;
use git2::{BranchType, Repository};

// We'll use a SelectView here.
//
// A SelectView is a scrollable list of items, from which the user can select
// one.

fn main() {
    let repo = match Repository::open("./") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let local_branch_option: Option<BranchType> = Some(BranchType::Local);

    let local_branches = match repo.branches(local_branch_option) {
        Ok(branches) => branches,
        Err(e) => panic!("failed to open: {}", e),
    };
    for result in local_branches {
        match result {
            Ok((b, _btype)) => match b.name() {
                Ok(n) => println!("{}", n.unwrap()),
                Err(e) => panic!("error! {}", e),
            },
            Err(e) => panic!("error! {}", e),
        }
    }

    let local_branches = match repo.branches(local_branch_option) {
        Ok(branches) => branches,
        Err(e) => panic!("failed to get branches: {}", e),
    };
    let mut branch_names: Vec<String> = Vec::new();
    for result in local_branches {
        match result {
            Ok((b, _btype)) => match b.name() {
                Ok(n) => branch_names.push(String::from(n.unwrap())),
                Err(e) => panic!("error! {}", e),
            },
            Err(e) => panic!("error! {}", e),
        };
    }

    let mut select: SelectView = SelectView::new()
        // Center the text horizontally
        .h_align(HAlign::Center)
        // Use keyboard to jump to the pressed letters
        .autojump();

    select.add_all_str(branch_names);

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

// Let's put the callback in a separate function to keep it clean,
// but it's not required.
fn show_next_window(siv: &mut Cursive, city: &str) {
    siv.pop_layer();
    let text = format!("let's checkout out {city}");
    siv.add_layer(Dialog::around(TextView::new(text)).button("Quit", |s| s.quit()));
}
