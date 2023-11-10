fn main() {
  println!("Hello, world!");
  /*
   *  rust声明变量默认是immutable的，所以不能二次赋值
   */
  // let x = 5;
  // println!("The value of x is: {x}");
  // x = 6;
  // println!("The value of x is: {x}");
  
  /*
   * 使用mut让变量可变
   */
  // let mut x = 5;
  // println!("The value of x is: {x}");
  // x = 6;
  // println!("The value of x is: {x}");

  /*
   * 常量可以在任何作用域中使用，包括全局
   * 常量只能用于常量表达式
   * rust要求常量命名在单词之间使用全大写加下划线
   */
  // const THREE_HOUSE_IN_SECONDS: u32 = 60 * 60 * 3;
  // println!("The value of THREE_HOUSE_IN_SECONDS is: {THREE_HOUSE_IN_SECONDS}");

  /*
   * 隐藏（Shadowing），变量可以重复声明，声明后将屏蔽声明该变量前的所有同名变量
   * 
   */

  // let x = 5;
  // let x = x + 1;
  
  // {
  //   let x = x + 2;
  //   println!("The value of x in the inner scope is: {x}");
  // }

  // println!("The value of x is: {x}");

  /*
  1、rust 数据类型分为：标量和复合
  2、rust是静态语言，编译时必须指定数据类型
  3、标量
    整型
    浮点型
    布尔类型
    字符串类型
   */

  // let sum = 5 + 10;
  // println!("The value of sum is: {sum}");
  
  // let difference = 95.5 - 4.3;
  // println!("The value of difference is: {difference}");

  // let product = 4 * 30;
  // println!("The value of product is: {product}");

  // let quotient = 56.7 / 32.2;
  // let truncated = -5 / 3;
  // println!("The value of quotient is: {quotient}");
  // println!("The value of truncated is: {truncated}");

  // let remainder = 43 % 5;
  // println!("The value of remainder is: {remainder}");

  /*
    复合类型：元组和数组
    元组，用于将多个其它类型的值进行组合；
    一旦声明，长度不会增大或缩小;
    使用解构取元组的值;
   */

  //  let tup: (i32, f64, u8) = (500, 6.4, 1);
  //  println!("The value of tup is: {tup}");

  // let tup = (500, 6.4, false);
  // let (x, y, z) = tup;
  // println!("The value of z is: {z} {x} {y}");


  // let tup: (i32, f64, u8) = (500, 6.4, 1);
  // let one = tup.0;
  // println!("The value of one is: {one}");

  /*
    数组长度固定且元素必须相同
   */

  //  let arr: [i32; 5] = [1, 2, 3, 4, 5];

  /*
  函数命名要求是小写，多个用下划线连接
  函数给返回值时，无需写分号，但必须声明函数的返回类型
   */

  // let x = five();

  // println!("The value of x is: {x}");

  // let y = {
  //   let x = 3;
  //   x + 1
  // };

  // println!("The value of y is: {y}");

  /*
  if else
   */
  // let number = 7;

  // if number > 5 {
  //   println!("big");
  // } else if number === 7 {
  //   println!("true");
  // } else {
  //   println!("small");
  // }

  // if返回类型必须相同
  // let condition = true;
  // let number = if condition { 5 } else { 6 };

  // println!("The value of number is: {number}");


  /*
   rust有三种循环:
    loop
    while
    for
   */

  // let mut counter = 0;

  // let result = loop {
  //   counter += 1;
    
  //   if counter == 10 {
  //     break counter * 2;
  //   }
  // };

  // println!("The value of result is: {result}");

  // let mut counter = 0;
  // 'counting_up: loop {
  //   println!("counter = {counter}");
  //   let mut remaining = 10;

  //   loop {
  //     println!("remaining = {remaining}");

  //     if remaining == 9 {
  //       break;
  //     }

  //     if counter == 2 {
  //       break 'counting_up;
  //     }

  //     remaining -= 1;
  //   };

  //   counter += 1;
  // }
  // println!("End counter = {counter}");

  // let mut number = 3;

  // while number != 0 {
  //   println!({"{number!}"});

  //   number -= 3;
  // }

  // println!("LIFTOFF!");
  // let a = [10, 20, 30, 40, 50];
  // let mut index = 0;

  // while index < 5 {
  //   println!("the value is {}", a[index]);
  //   index += 1;
  // }

  // let a = [10, 20, 30, 40, 50];
  
  // for el in a {
  //   println!("the value is: {}", el);
  // }

  for number in (1..4).rev() {
    println!("{number}");
  }
  println!("LIFTOFF!!!");

}

/*
  函数返回值
*/
// fn five() -> i32 {
//   5
// }

/*
错误示例，函数给返回值时，无需写;
*/
// fn plus_on(x: i32) -> i32 {
//   x + 1;
// }