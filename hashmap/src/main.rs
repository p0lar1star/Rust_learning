use std::collections::HashMap;

fn main() {
    // println!("Hello, world!");
    // // 创建hashmap
    // // let mut scores: HashMap<String, i32> = HashMap::new();
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);

    // 使用collect从vector创建hashmap
    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];
    // // 由于collect可以返回很多种类型，必须显式指明返回的是HashMap
    // let scores:HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // for i in teams.iter() {
    //     println!("{}", i);
    // }

    // for i in (0..2).rev() {
    //     let t = match teams.get(i) {
    //         Some(value) => value,
    //         None => {
    //             println!("Out of index");
    //             "out of index"
    //         }
    //     };
    //     println!("{}", t);
    // }

    // get()方法得到HashMap中的值
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);
    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name);// 注意这里传递的是引用而不是值
    // match score {
    //     Some(value) => println!("{}", value),
    //     None => println!("team not exist"),
    // };

    // // 遍历，注意使用HashMap的引用
    // for(k, v) in &scores {
    //     println!("{}, {}", k, v);
    // }


    // 更新HashMap
    // 如果向HashMap中插入一对KV，然后再插入同样的K，但是不同的V，那么原来的V会被替换（覆盖）掉
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Blue"), 25);
    println!("{:#?}", scores);

    // 只在k不对应任何值的情况下，才插入v
    // 使用entry方法，检查指定的k是否对应一个v
    // 参数为k，返回enum entry：代表值是否存在，不存在则使用or_insert()插入
    // or_insert()若k存在则返回对应的v的一个可变引用
    // 若k不存在则先插入再返回到这个值的可变引用
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Green")).or_insert(100);
    println!("{:#?}", scores);
    
    // 改变HashMap中的值
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
        println!("{}", word);
    }
    println!("{:#?}", map);
    
}
