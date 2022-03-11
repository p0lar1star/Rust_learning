// trait
use trait_::NewsArticle;
use trait_::Summary;// 导入trait


fn main() {
    let article = NewsArticle {
        headline: String::from("123"),
        content: String::from("456"),
        author: String::from("789"),
        location: String::from("91011"),
    };

    println!("1 new Tweet: {}", article.summarize());
}
