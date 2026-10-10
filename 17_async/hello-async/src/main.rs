use trpl::Html;

/*
* usage
*   cargo run -- "https://www.rust-lang.org"
*
* - In async, rust does nothing unless you add the await keyword (unlike other languages)
* - "futures are lazy"
*
* - async code needs a runtime
* - there are many diff runtimes (some for diff use-cases)
*/

// Example state machine the async runtime runs:
// 1. Starts one async task, hits await and looks for another async task that might need started
// 2. Starts the other async task
// 3. Once first await finishes, pucks back up on that one
enum PageTitleFuture<'a> {
    Initial { url: &'a str },
    GetAwaitPoint { url: &'a str },
    TextAwaitPoint { response: trpl::Response },
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // block on starts an async runtime under hood
    // it runs the future returned by async block
    trpl::block_on(async {
        let title_future_1 = page_title(&args[1]);
        let title_future_2 = page_title(&args[2]);

        let (url, maybe_title) = match trpl::select(title_future_1, title_future_2).await {
            trpl::Either::Left(left) => left,
            trpl::Either::Right(right) => right,
        };

        println!("{url} returned first");

        match maybe_title {
            Some(title) => println!("title for url {url} was {title}"),
            None => println!("url {url} had no page title"),
        }
    });

    // TODO: left off https://doc.rust-lang.org/book/ch17-01-futures-and-syntax.html#racing-two-urls-against-each-other-concurrently
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    // let response = trpl::get(url).await;
    // let response_text = response.text().await;

    // could chain them together like this as well:
    let response_text = trpl::get(url).await.text().await;

    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html()); // only runs if select_first returns Option::Some (could use match too, but .map more idiomatic)

    (url, title)
}

// Block marked with async - compiles to "anonymous datatype" that implements the Future trait
// Function marked with async - compiles to non-async func who's body is async block
// Async func's return type  is the "anonymous datatype"
//
// async block -> anonymous_dt<future>
// async func -> body -> async block
//   - returns anonymous_dt<future>
//
// So, async fn is similar to writing non-async func that returns impl Future<Output = Option<String>>
// like below:
fn page_title_for_demonstration(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        let response_text = trpl::get(url).await.text().await;

        Html::parse(&response_text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}
