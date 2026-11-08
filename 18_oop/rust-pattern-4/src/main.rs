use rust_pattern_4::Post;

fn main() {
    let mut post = Post::new(); // creates a DraftPost

    post.add_text("I ate salad for lunch today");
    // DraftPost doesn't have a content method - so instead of returning empty string
    // this pattern just removes the method altogether to prevent draft post from accidentally
    // showing content.
    // assert_eq!("", post.content()); 

    // TODO: left off https://doc.rust-lang.org/book/ch18-03-oo-design-patterns.html#encoding-states-and-behavior-as-types
    // See: "updated code in main is shown in Listing 18-21"
}
