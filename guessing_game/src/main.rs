use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("猜数");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("神秘数字是：{}", secret_number);

    loop {
        println!("猜一个数");
    
        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess).expect("无法读取行!");
    
        println!("你猜测的数是: {}", guess);
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入数字");
                continue;
            }
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小啦！"),
            Ordering::Greater => println!("太大啦！"),
            Ordering::Equal => {
                println!("对啦！");
                break;
            },
        }
    }
}
