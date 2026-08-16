// define a trait (interface)
pub trait Summary {
    fn summaraze(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// NewsArticle implments Summary trait
impl Summary for NewsArticle {
    fn summaraze(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

// SocialPost implments Summary trait
impl Summary for SocialPost {
    fn summaraze(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Note - can only add a trait to a type if:
// - The trait or the type are defined locally in your project
// - Or both are defined locally.
// If both are defined outside your project. You can't add a trait to a type
// - "You can't add an externally defined trait to an externally defined type"
// TODO: left off https://doc.rust-lang.org/book/ch10-02-traits.html#using-default-implementations
