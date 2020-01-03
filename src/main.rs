use reqwest;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://news.ycombinator.com/").await?;
    let body = resp.text().await?;
    let fragment = Html::parse_document(&body);
    let selector = Selector::parse("a.storylink").unwrap();
    for story in fragment.select(&selector) {
        let story_text = story.text().collect::<Vec<_>>();
        println!("{:?}", story_text);
    }
    Ok(())
}
