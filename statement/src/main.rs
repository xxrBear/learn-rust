fn main() {
    let y = {
        let x = 1;
        x + 1 // 加;会报错，加了就不是表达式了
    };
    println!("y is {}", y);
}
