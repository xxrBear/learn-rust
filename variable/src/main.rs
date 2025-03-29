fn main() {
    // 声明一个变量
    // let number = 1;
    // println!("this number is {}", number);

    // number = 2;
    // let mut number = 1;
    // println!("this number is {}", number);
    // number = 2;
    // println!("this number is {}", number);

    let _x = 1; // 忽略变量

    // 变量遮蔽
    let x = 1;
    let x = x + 1;
    println!("this x is {}", x);
}
