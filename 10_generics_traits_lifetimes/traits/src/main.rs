use aggregator::{SocialPost, Summary};
// aggregator defined in 10_generics_traits_lifetimes/aggregator

fn main() {
    // Traits define functionality a type has.
    // Traits allow us to share functionality with other types
    // They are sort of like interfaces in other languages

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    let summary = post.summaraze();

    println!("post summary: {summary}");
}
