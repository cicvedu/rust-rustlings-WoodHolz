// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    call_me(3);
}

// 调用函数的参数必须说明其类型
fn call_me(num:i32) {
    for i in 0..num { // 学习一下循环的语法
        println!("Ring! Call number {}", i);
    }
}
