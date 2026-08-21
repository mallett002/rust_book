// Lifetimes ensure references are valid as long as we need them to be
// Every reference has a lifetime
fn main() {
    dangling_refs();
    generic_lifetimes_in_funcs();
    lifetimes_in_structs();
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

    // Example with different scopes (lifetimes)
    // Compiler approves: shortest lifetime is long enough (where result used)
    let string1 = String::from("long string is long");

    {
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
        println!("Longest is {result}");
    }

    // // Example where compiler doesn't approve
    // // Longest lifetime says result needs to live as long as the shortest of 2 params
    // // Although the actual reference used (string1) would be used, the lifetime of string2 is
    // // picked (bc it's the shortest)
    // // Rust is saying the reference might not live long enough (longest doesn't know which will be
    // // picked, x or y at compile time)
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {result}");
}

// Only adding lifetime to one param (x) and return type
// This compiles and is fine
// LT of y doesn't have relationship with x nor return val
fn longest_2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// // lifetime only on the return val, not any params
// // This would return a dangling ref (compile error)
// fn longest_3<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("Really long string");
//     result.as_str()
// }

// Lifetimes on structs
// 'a here means data in part will live as long as struct instance does
// ie: struct instance won't outlive data in "part"
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn lifetimes_in_structs() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split(".").next().unwrap();

    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
