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

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(async {
        let url = &args[1];

        match page_title(url).await {
            Some(title) => println!("title for url {url} was {title}"),
            None => println!("url {url} had no page title"),
        }
    });
}

async fn page_title(url: &str) -> Option<String> {
    // let response = trpl::get(url).await;
    // let response_text = response.text().await;

    // could chain them together like this as well:
    let response_text = trpl::get(url).await.text().await;

    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html()) // only runs if select_first returns Option::Some (could use match too, but .map more idiomatic)
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
