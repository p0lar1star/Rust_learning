// 若要读取命令行参数，需要用到标准库函数std::env::args()
// args()返回的是一个迭代器，还须使用collect()将其转化为集合类型，如vector
// 为了指定转化成哪一种集合类型，需要显式的声明
// 如：let args: Vec<String> = env::args().collect();
use std::env;
// exit(1)退出程序，需要用到标准库std::process
use std::process;
// 导入自建的lib
use minigrep::Config;

// cargo run xxxx(命令行参数)
fn main() {
    // 接收命令行参数部分
    let args: Vec<String> = env::args().collect();
    // 若命令行参数包含非法unicode字符，需使用env::args_os()，返回的是OsString类型
    // println!("{:#?}", args);
    // let query = &args[1];
    // let filename = &args[2];
    // println!("Search for {} in file {}", query, filename);
    // unwrap_or_else:若返回Result为Ok(x)，则取出Ok中的值给config
    // 否则执行参数——匿名函数
    let config = Config::new(&args).unwrap_or_else(|err| {
        // 输出到标准错误
        eprintln!("Problem parsing arguments: {}", err); // 确保程序异常终止时只有一句话，没有多余的错误信息
        process::exit(1);
    });

    // 读取文件部分
    // let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    // println!("With text:\n{}", contents);
    if let Err(err) = minigrep::run(config) {
        // 标准错误
        eprintln!("Application error: {}", err);
        process::exit(1);
    };
}
