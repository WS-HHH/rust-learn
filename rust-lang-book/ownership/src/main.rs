fn main() {
  /*
    rust中所有变量都有一个所有者，
    值在任何时候有且仅有一个所有者
    当所有者离开作用域时，变量将被销毁
   */
  // println!("Hello, world!");
  // let s1 = String::from("hello");
  // // let s2 = s1;
  // let s2 = s1.clone();
  // println!("The value of s1 is:{}", s1);

  // 转移返回值所有权示例：
  // let s1 = gives_ownership();
  // println!("The value of s1 is: {}", s1);

  // let s2 = String:: from("s2");

  // let s3 = takes_and_gives_back(s2);
  // println!("The value of s3 is: {}", s3);

  // 返回参数的所有权示例
  let s1 = String::from("hello");

  let (s2, length) = calculate_value(s1);

  println!("The value of s2 and length is: {},{}", s2, length);

}

// fn gives_ownership() -> String {
//   let s1 = String::from("your");

//   s1
// }

// fn takes_and_gives_back(s2: String) -> String {
//   s2
// }


// fn calculate_value(s1: String) -> (String, usize) {
//   let length = s1.len();

//   (s1, length)

// }