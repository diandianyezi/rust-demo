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
  let price2:u32 = 200;
  let price3:i32 = -300;
  let price2:isize = 400;
  let price2:usize = 500;

  // 当值与类型不匹配时，编译会报错
  let price7:i8 = 192;
}