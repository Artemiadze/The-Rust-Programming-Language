use trpl::{Either, Html};

fn main() {
    futures();
    println!("--------------------------");
}

fn futures(){
    // cargo run https://google.com https://rust-lang.org
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        // Two asynchronous tasks: loading pages
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        let (url, maybe_title) =
            match trpl::race(title_fut_1, title_fut_2).await { // runs both asynchronous tasks in parallel 
            // and returns the result of the one that completed first.
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    })
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}