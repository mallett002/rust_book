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

    // State has internal method "request_review" that has different functionality
    // depending on which state it's currently in.
    // Self.state is Option bc we can't have an empty field on struct (take leaves a Option::None)
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() { // replace state temporarily with None
            self.state = Some(s.request_review()) // new state goes in
        }
    }
}

// State trait - defines funcitonality for Boxed self types (boxed state)
// Takes ownership of the boxed state - invalidating the old state
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

// Draft state - moves draft state into pending review state
struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

// Pending Review state - request review just keeps it in pending review
struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
