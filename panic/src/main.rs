use std::fs::File;
use std::io::ErrorKind;

fn main() {

    // let result = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Error opening file {:#?}", error);
    //     }
    // };

    
    // unwrap
    // let false_alarm_1 = File::open("hello.txt").unwrap();

    
    // except
    let false_alarm_2 = File::open("hello.txt").expect("无法打开文件 hello.txt");
    
    // 复杂的match
    // let f = File::open("hello.txt");
    // let false_alarm = match f {
    //     Ok(file) => file,
    //     // 出错时，error.kind()方法，返回错误类型
    //     Err(error) => match error.kind() {
    //         // 当错误类型是NotFound时，创建文件
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Error creating file: {:#?}", e),
    //         },
    //         // 当错误类型是其他类型时，panic
    //         othererror => panic!("Error opening and creating file:{:#?}", othererror),
    //     }
    // };
}