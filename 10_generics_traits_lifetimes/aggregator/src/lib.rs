// Note - can only add a trait to a type if:
// - The trait or the type are defined locally in your project
// - Or both are defined locally.
// If both are defined outside your project. You can't add a trait to a type
// - "You can't add an externally defined trait to an externally defined type"

use std::fmt::Debug;
use std::fmt::Display;

// define a trait (interface)
pub trait Summary {
    fn summarize_author(&self) -> String;

    // default impl:
    // can call other methods in same trait:
    fn summarize(&self) -> String {
        format!("Read more {}...", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// NewsArticle implments Summary trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

// implement the default (example)
// impl Summary for NewsArticle {}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

// SocialPost implments Summary trait
impl Summary for SocialPost {
    // comment out to use default impl
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Using traits as parameters
// Accepts anything that implements the Summary trait
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound syntax (longer form of the "traits as parameters")
fn notify_2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// allow 2 parameters that implement Summary
fn notify_3(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

// Force both parameters to have the same concrete type (have to use Trait Bound syntax)
fn notify_4<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

// Ensure parameter implements more than one trait
fn notify_5(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}

// Ensure parameter implements more than one trait with Trait bounds
fn notify_6<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Using multiple Trait Bounds with Where syntax
// Cluttered and hard to read:
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    5
}

// improved with Where syntax
fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    5
}

// Returns something that implements Summary:
fn returns_summerizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    }
}

// Won't work: Can't return 2 different concrete types even if both impl Summary
// fn returns_summerizable_wont_work(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         SocialPost {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             repost: false,
//         }
//     }
// }

// Using Trait Bounds to Conditionally Implement Methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Only implements cmp_display if T implements Display AND PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The greatest member is x = {}", self.x);
        } else {
            println!("The greatest member is y = {}", self.y);
        }
    }
}

// Blanket implementations
// Ex: Put ToString on anything that implements Display (already exists in std lib)
// impl<T: Summary> ToString for T {
//     // --snip--
// }

// example for blanket implementation
pub trait AsSummaryString {
    fn as_summary_string(&self) -> String;
}

// Blanket impl: Add AsSummaryString to anything that impls Summary
impl<T: Summary> AsSummaryString for T {
    fn as_summary_string(&self) -> String {
        self.summarize()
    }
}
