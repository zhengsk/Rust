use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("猜数");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("神秘数字是：{}", secret_number);

    println!("猜一个数");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("无法读取行!");

    println!("你猜测的数是: {}", guess);

    let guess: u32 = guess.trim().parse().expect("请输入一个数字");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("太小啦！"),
        Ordering::Equal => println!("对啦！"),
        Ordering::Greater => println!("太大啦！"),
    }
}
