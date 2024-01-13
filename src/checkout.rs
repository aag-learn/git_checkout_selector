use crate::local_repository::LocalRepository;
pub mod local_repository;

fn main() {
    match LocalRepository::default().checkout("test1") {
        Ok(_) => println!("All good!!"),
        Err(e) => panic!("{}", e.message()),
    }
}
