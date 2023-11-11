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
  // let s1 = String::from("hello");

  // let (s2, length) = calculate_value(s1);

  // println!("The value of s2 and length is: {},{}", s2, length);

  // 引用示例
  // let s1 = String::from("hello");
  // let len = calculate_len(&s1);
  // println!("The value of is {}", len);

  // 可变引用示例
  // let mut s1 = String::from("hello");
  // change(&mut s1);
  // println!("The value of is {}", s1);

  // 如果对某变量进行了可变引用，无法同时再对改变量进行引用
  // let mut s1 = String::from("hello");
  // let s2 = &mut s1;
  // let s3 = &mut s1; // 不可用

  
  // println!("The value of is s2 and s3, {}, {}", s2, s3);
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

// fn calculate_len(s1: &String) -> usize {
//   s1.len()
// }

// 可变引用
fn change(s1: &mut String) {
  s1.push_str(", world");
}
