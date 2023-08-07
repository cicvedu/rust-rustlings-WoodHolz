// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.


// 如果调用函数定义有参数, 调用该函数时必须填入参数
fn main() {
    call_me(1);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i);
    }
}
