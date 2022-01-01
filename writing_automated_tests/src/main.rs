mod how_to_organize_tests_files;
mod how_to_run_tests;
mod how_to_write_tests;

use how_to_organize_tests_files::entry as how_to_organize_tests_files_entry;
use how_to_run_tests::entry as how_to_run_tests_entry;
use how_to_write_tests::entry as how_to_write_tests_entry;

fn main() {
    how_to_write_tests_entry();
    how_to_run_tests_entry();
    how_to_organize_tests_files_entry();
}
