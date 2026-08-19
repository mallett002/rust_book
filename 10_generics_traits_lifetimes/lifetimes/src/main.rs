// Lifetimes ensure references are valid as long as we need them to be
// Every reference has a lifetime
fn main() {
    dangling_refs();
    generic_lifetimes_in_funcs();
}

fn dangling_refs() {
    // main aim of lifetimes - prevent dangling refs
    let r;

    {
        let x = 5;
        r = &x; // borrowing x here.
    }

    // r is ref to x, but x died - causes compile error
    // println!("r = {r}");


    // Fix 
    let a;
    let b = 5;
    a = &b; // borrowing x here.

    // r is ref to x, but x died
    println!("a = {a}"); // problem - r 
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

fn generic_lifetimes_in_funcs() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    // TODO: left off https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotation-syntax
}

