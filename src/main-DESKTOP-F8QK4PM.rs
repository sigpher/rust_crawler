use jsonpath_lib as jsonpath;
use reqwest;
use scraper::{Html, Selector};
use serde_json::json;
#[tokio::main]

async fn main() -> Result<(), reqwest::Error> {
    let mut count = 0;
    // for page in 0..10 {
    //     let url = format!(
    //         "https://movie.douban.com/top250?start={}&filter=",
    //         page * 25
    //     );
    //     let client = reqwest::Client::new();
    //     let resp = client.get(url).send().await?;

    //     let body = resp.text().await?;
    //     let doc = Html::parse_fragment(&body);
    //     let selectors = "#content > div > div.article > ol > li > div > div.info > div.hd > a > span:nth-child(1)";
    //     let selector = Selector::parse(selectors).unwrap();
    //     for el in doc.select(&selector) {
    //         count += 1;
    //         println!("{:03}:{}", count, el.inner_html());
    //     }
    // }

    let url = "http://jsonplaceholder.typicode.com/photos";
    let client = reqwest::Client::new();
    let resp: serde_json::Value = client.get(url).send().await?.json().await?;
    let json = json!(resp);
    let mut selector = jsonpath::selector(&json);
    let id = selector("$..id").unwrap();
    let title = selector("$..title").unwrap();
    let ret: Vec<_> = id.iter().zip(title.iter()).collect();
    println!("{:?}", ret);
    Ok(())
}
