fn main() {
    // Demonstrating "move"
    let s1 = String::from("hello");
    let s2 = s1; // move
    // println!("{s1}"); // Error can't do this (this has "moved")
    println!("{s2}");

    // vars no longer needed are dropped
    let mut s = String::from("hello"); // allocation moved (dropped) once reassigned below
    println!("{s} world");
    s = String::from("ahoy!"); // causes above "s" to be dropped
    println!("{s} world");
}
