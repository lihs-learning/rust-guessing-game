use std::io;

fn main() {
    println!("猜数字！");

    println!("请输入你猜的数字。");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("没有读到任何内容");

    println!("你猜的数字是：{}", guess);
}
