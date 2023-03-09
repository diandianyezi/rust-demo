fn main() {
  // let 变量名 = 值
  let food = "清蒸";
  let price = 1232;
  let checked = true;

  println!("food is {}", food);
  println!("food is {}", price);
  println!("food is {}", checked);

  // 有符号类型signed（可以存储正数或负数） 无符号类型usigned只能存储正数
  // 按存储空间来说，划分为1字节 2字节 4字节 8字节 16字节，1字节= 8位
  // i32是默认的整型

  // let price2:u32 = 200;
  // let price3:i32 = -300;
  // let price4:isize = 400;  // arch类型 有符号
  // let price5:usize = 500;  // arch类型 无符号

  // 当值与类型不匹配时，编译会报错

  // let price76:i8 = 16.22;
  // let price7:i8 = 192;

  // let price7:f32 = 199.0;
  // let price8:f64 = 199.0;

  // let s:char = 'O'; // 字符类型，单个字符或字 UTF-8作为底层编码，包含数字、字母、Unicode和其他特殊字符

  // 常量
  // const PI:f64 = 3.1415926;
  static PI1:&'static str = "hi";
  println!("{}", PI1);

  // &str 是在模块 std:str,字符串切片
  // let lesson: &str = "学习Rust";

  //
   // 字符串对象
   // String:new() 创建一个新的空字符串，静态的
   // String::from() 从具体的字符串字面量创建字符串对象。
   //

  let s1:String = String::new();
  println!("s1:{},s1-len:{}", s1,s1.len());

  let s2:String = String::from("学习吧");
  println!("s2:{},s2-len:{}", s2, s2.len());

  let mut s3:String = String::new();
  s3.push_str("测试");
  println!("{}", s3);

  s3.push('Q');
  s3.push('W');
 
  println!("{}", s3);
}