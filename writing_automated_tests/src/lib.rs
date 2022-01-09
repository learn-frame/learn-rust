mod how_to_organize_tests_files;
mod how_to_run_tests;
mod how_to_write_tests;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub use how_to_organize_tests_files::add_group;
