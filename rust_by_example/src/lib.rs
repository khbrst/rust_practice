mod custom_types;
mod hello_world;
mod primitives;

pub fn run_hello_world() {
  hello_world::print_hello_world();
  hello_world::formatted_print();
  hello_world::debug_print();
  hello_world::pretty_print();
  hello_world::display_print();
  hello_world::display_print_for_list();
  hello_world::formatting_print();
}

pub fn run_primitives() {
  primitives::check_basics();
  primitives::check_literals_and_operators();
  primitives::check_tuples();
  primitives::check_arrays_and_slices();
}

pub fn run_custom_types() {
  custom_types::check_structs();
  custom_types::check_enums();
  custom_types::use_enums();
  custom_types::c_like_enums();
  custom_types::linked_list_enums();
  custom_types::check_constants();
}
