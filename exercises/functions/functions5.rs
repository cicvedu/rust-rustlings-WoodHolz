// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

// 本题就是在考查语句和表达式的区别, 说明详见`function1.rs`添加的注释
fn square(num: i32) -> i32 {
    num * num
}
