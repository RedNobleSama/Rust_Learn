use std::io; // prelude
use std::cmp::Ordering;
use rand::Rng;
use std::intrinsics::copy_nonoverlapping; // trait

fn main() {
    println!("猜数");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("神秘数字是{}", secret_number);
    loop {
        println!("猜测一个数");
        let foo = 1;
        let bar = foo; //所有变量不可变,加mut 变量可变
        let mut guess = String::new(); // 定义一个字符串变量，并初始化对象
        io::stdin().read_line(&mut guess).expect("无法读取行");
        // io::Result Ok, Err

        // shadow,trim()去除字符串空白值,parse()
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你猜测的数是: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("to big"),
            Ordering::Equal => {
                println!("you win");
                break;
            },
        }
    }

}
