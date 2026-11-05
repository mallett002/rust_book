pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        ""
    }
}

trait State {}

struct Draft {}

impl State for Draft {}

// TODO: left off https://doc.rust-lang.org/book/ch18-03-oo-design-patterns.html#ensuring-that-the-content-of-a-draft-post-is-empty
