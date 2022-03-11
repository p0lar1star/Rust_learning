// 运算符重载
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// 重载：亮个相同类型的相加
// pub trait Add<Rhs = Self>
// fn add(self, rhs: Rhs) -> Self::Output;
impl Add for Point {
    type Output = Point; // The resulting type after applying the `+` operator.
                         // 重载+运算符
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, PartialEq)]
// tuple type没有名字，它的元素也没有名字
// struct type有名字，它的元素也有名字
// tuple struct type有名字，它的元素没有名字
struct Millimeters(u32);
struct Meters(u32);
// 两个不同类型的相加：毫米和米相加，需要具体指明泛型参数类型
impl Add<Meters> for Millimeters {
    // 定义相加后得到的类型
    type Output = Millimeters;
    // fn add(self, Meters) -> Millimeters
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + other.0 * 1000)
    }
}

// 完全限定语法：调用同名的方法
struct Human;
pub trait Pilot {
    fn fly(&self);
}

pub trait Wizard {
    fn fly(&self);
}

impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot!");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard!");
    }
}

impl Human {
    fn fly(&self) {
        println!("fly!");
    }
}

struct dog;

impl dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

trait Animal {
    fn baby_name() -> String;
}

impl Animal for dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("{:#?}", Point { x: 1, y: 0 } + Point { x: 2, y: 3 });
    println!("{:?}", Millimeters(1) + Meters(1));
    // println!("{:?}", Meters(1) + Millimeters(1));// error

    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);

    // <Type as Trait>::function_name(arg1, arg2, ...)
    println!("{}", dog::baby_name());
    println!("{}", <dog as Animal>::baby_name());
}
