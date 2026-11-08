use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate salad for lunch today", post.content());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_text_request_review_and_approve() {
        let mut post = Post::new();

        post.add_text("I ate salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("I ate salad for lunch today", post.content());
    }

    // was going to implement the reject method here but decided to just move on
    #[test]
    fn should_reject_post() {
        let mut post = Post::new();

        post.add_text("I ate salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        // post.reject();
        assert_eq!("", post.content());

    }
}

