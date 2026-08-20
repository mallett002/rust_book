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



// lifetime 'a: the returned ref will be valid as long as both params are valid (no dangling refs)
// tells compiler: "reject code if return value could outlive any input"
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
// Lifetimes in longest func ^^
// params will live at least as long as 'a (picks the shortest lived on and assigns to 'a)
// return value will live at least as long as 'a

fn generic_lifetimes_in_funcs() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);

    // example lifetimes:
    // &i32 -> just a reference
    // &'a i32 -> ref with explicit lifetime
    // &'a mut i32 -> mutable ref with explicit lifetime

    // TODO: left off https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#in-function-signatures
    // Find text: "Let’s look at how the lifetime annotations restrict the longest function by passing in references that have different concrete lifetimes"
}

