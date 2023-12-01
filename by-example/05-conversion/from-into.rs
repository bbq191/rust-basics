use std::convert::From;
use std::convert::Into;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}
// 你的类型实现了 From，那么同时你也就免费获得了 Into。
// 使用 Into trait 通常要求指明要转换到的类型，因为编译器大多数时候不能推断它。不过考虑到我们免费获得了 Into，这点代价不值一提。

// impl Into<Number> for i32 {
//     fn into(self) -> Number {
//         Number { value: self }
//     }
// }
fn main() {
    let my_str = "hello";
    let my_string = String::from(my_str);
    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    // Try removing the type annotation
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
