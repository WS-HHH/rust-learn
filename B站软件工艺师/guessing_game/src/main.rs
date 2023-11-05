use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
  
  // lesson 2.1
  // println!("Hello, world!");
  // println!("猜数！");

  // println!("猜数！");
  // let mut guess = String::new();

  // io::stdin().read_line(&mut guess).expect("无法读取行");

  // println!("你猜测的数是：{}", guess);


  // lesson 2.2 生成神秘数字
  // println!("猜测一个数");

  // let mut guess = String::new();

  // io::stdin().read_line(&mut guess).expect("无法读取行");

  // println!("你猜测的数是：{}", guess);
  // let secret_number = rand::thread_rng().gen_range(1, 101);
  // println!("神秘数字是：{}", secret_number);

  // lesson 2.3 比猜测的数字和输入数字的大小

  // println!("猜测一个数");

  // let mut guess = String::new();

  // io::stdin().read_line(&mut guess).expect("无法读取行");

  // println!("你猜测的数是：{}", guess);
  // let secret_number = rand::thread_rng().gen_range(1, 101);
  // println!("神秘数字是：{}", secret_number);

  // let guess: u32 = guess.trim().parse().expect("please type a number");
  // println!("You guess number is: {}", guess);

  // match guess.cmp(&secret_number) {
  //   Ordering::Less => println!("too small"),
  //   Ordering::Greater => println!("too big"),
  //   Ordering::Equal => println!("you win"),
  // }

  // lesson 2.4 多次猜测

  loop {
    println!("猜测一个数");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("无法读取行");
  
    println!("你猜测的数是：{}", guess);

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("神秘数字是：{}", secret_number);
  
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };
    println!("You guess number is: {}", guess);
  
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("too small"),
      Ordering::Greater => println!("too big"),
      Ordering::Equal =>  {
        println!("you win");
        break;
      },
    }
  }
}
