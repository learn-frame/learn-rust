pub mod hashmap;
pub mod string;
pub mod vector;

fn main() {
    vector::learn_vector();

    string::learn_string();
    string::update_string();
    string::index_string();
    string::slice_string();
    string::traverse_string();

    hashmap::learn_hashmap();
}
