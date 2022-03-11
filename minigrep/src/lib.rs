// 处理文件相关的事务，需用到标准库中的std::fs
use std::fs;
// exit(1)退出程序，需要用到标准库std::process
use std::error::Error;
// 使用环境变量，要用到std::env
use std::env;

// 不够rusty
// fn parse_config<'a>(args: &'a Vec<String>) -> (&'a str, &'a str) {
//     let query = &args[1];
//     let filename = &args[2];
//     (query, filename)
// }

// 更加rusty
pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        // str是静态生命周期
        if args.len() < 3 {
            return Err("No enough arguments!");
        }
        // let quert = args[1].clone();
        let query = args[1].clone();
        let filename = args[2].clone();
        // 使用环境变量，env::var()中传入环境变量的名称，返回Reselt
        // 如果CASE_INSENSITIVE被设置了，那么就返回Ok(),否则返回Err()
        // CASE_INSENSITIVE=1 cargo run flag flag.txt能找到FLAG
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        println!("Search for {} in file {}", query, filename);
        println!("{}", args[0]);
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// 在不同的场景下可以返回不同的错误类型
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ?在遇到错误时不会直接panic，而是将错误返回给函数的调用者，给调用者处理
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);
    let results = if config.case_sensitive {
        search_case_sensitive(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    println!("\nFind:");
    if results.len() == 0 {
        println!("No results!");
    }
    for result in results {
        println!("{}", result);
    }
    Ok(())
}

// pub fn search(query: &str, contents: &str) -> Vec<String> {
//     let mut v: Vec<String> = Vec::new();
//     for line in contents.lines() {
//         if line.contains(query) {
//             v.push(line.to_string());
//         }
//     }
//     v
// }

// 区分大小写的搜索：严格匹配
pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

// 不区分大小写的搜索
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    // 让query变成小写，注意query_lowercase是具有所有权的String，因为to_lowercase()会创建一个新的String
    let query_lowercase = query.to_lowercase();
    for line in contents.lines() {
        // 让每一行变成小写，注意contains要求传入的是一个字符串切片
        if line.to_lowercase().contains(query_lowercase.as_str()) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    // 引入外部模块中所有的内容
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three,
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
