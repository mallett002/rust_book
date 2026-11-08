// starts as DraftPost
pub struct DraftPost {
    content: String,
}

// request_review turns draft into PendingReviewPost
pub struct PendingReviewPost {
    content: String,
}

// PendingReviewPost turns into represents a published Post when approved
pub struct Post {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // takes ownership of self - no lingering DraftPost
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    // takes ownership of self - no lingering PendingReviewPost
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

impl Post {
    // new post just creates a DraftPost
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}
