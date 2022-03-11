#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

//在impl块里定义方法
//方法的第一个参数可以是&self，也可以获得其所有权(self) 或 可变借用(&mut self)
impl Rectangle {
    fn area(&self) -> u32 {
        (*self).width * self.length
    }
    //关联函数, 第一个参数不是该结构体自身、自身的引用或可变引用
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            length : size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 300,
        length: 500,
    };
    println!("The area of rectangle is {}", rect.area());

    let s = Rectangle::square(20);
    println!("The area of square is {}", s.area());
}
