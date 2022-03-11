// trait是一组动作的集合，类似于接口
// trait抽象的定义了共享行为
// 不同类可以有相同的动作
pub trait Summary {
    fn summarize_author(&self) -> String;
    // trait中summarize方法的默认实现
    fn summarize(&self) -> String {
        format!("Read more from {} ...", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
    // 一定要完整地实现trait
    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
}
