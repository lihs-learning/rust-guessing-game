// 将其他库里的代码功能导入到当前作用域
use rand::{Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数字！");

    let secret_number = rand::thread_rng().gen_range(1..101); // 左闭右开

    // println!("秘密数字为：{}", secret_number);

    // 无限循环
    loop {
        println!("请输入你猜的数字：");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("没有读到任何内容");

        // 作用域遮盖，而【不是】覆盖！两个变量都还在。
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜的数字是：{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("猜小了！"),
            Ordering::Greater => println!("猜大了！"),
            Ordering::Equal => {
                println!("你赢了！");
                break;
            },
        }
    }
}
