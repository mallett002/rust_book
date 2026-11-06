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

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        // content method to delegate to method on its state
        self.state.as_ref().unwrap().content(self)
        // use as_ref() bc we need a ref (can't move state out of the borrowed &self)
        // unwrap the option to get the actual state and call content on it, passing in the self
    }

    // State has internal method "request_review" that has different functionality
    // depending on which state it's currently in.
    // Self.state is Option bc we can't have an empty field on struct (take leaves a Option::None)
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            // replace state temporarily with None
            self.state = Some(s.request_review()) // new state goes in
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

// State trait - defines funcitonality for Boxed self types (boxed state)
// Takes ownership of the boxed state - invalidating the old state
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;

    // default implementation (for Draft and PendingReview states)
    fn content<'a>(&self, _post: &'a Post) -> &'a str { // lifetime: returned content will be valid as long as the post is
        ""
    }
}

// Draft state - moves draft state into pending review state
struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

// Pending Review state
struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // approving from pending review state moves it into published
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

// Published state
struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
