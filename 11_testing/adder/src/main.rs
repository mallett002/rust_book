// Often times, if you want a binary crate (not library crate), it's easier to test via a library
// crate. So, we make the libray crate first, and then create a main.rs that calls into the library
// crate to start the program, or run the code. Then, when testing via integration tests, we test
// directly into the library code, and skip the main.rs (non-lib code). The main.rs is just a thin
// wrapper that calls the library code, to just expose it as a binary program
fn main() {
    adder::add(2, 2 );
}
