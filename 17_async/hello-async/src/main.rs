use trpl::Html;

/*
* - In async, rust does nothing unless you add the await keyword (unlike other languages)
* - "futures are lazy"
*/

fn main() {
    println!("Hello, world!");
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
