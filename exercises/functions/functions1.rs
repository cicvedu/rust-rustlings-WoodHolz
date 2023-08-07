// functions1.rs
//
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a
// hint.


// 调用函数的语句加不加`;`都能过, 需要看一下相关的知识点

/**
 * 在Rust中, 调用函数时可以选择是否在语句末尾加上分号(;)。这涉及到Rust中表达式和语句的区别。
 * 
 * 在Rust中, 表达式（Expression）是生成某个值的代码片段, 而语句（Statement）是执行某个操作但不返回值的代码片段。
 * 
 * 当你在调用函数时, 如果在语句末尾添加了分号, 表示你将该函数调用作为一个语句来执行, 而不关心其返回值。这通常用于执行一些操作, 例如打印信息或进行某些副作用。
 * 
 * 例如：
 * 
 * ``` rust
 * copy
 * fn main() {
 *     println!("hello, world!");  // 函数调用作为语句执行
 *     another_function();          // 函数调用作为语句执行
 * }
 * 
 * fn another_function() {
 *     println!("another function.");
 * }
 * ```
 * 另一方面, 如果在函数调用后没有添加分号, 表示你将函数调用作为一个表达式来执行, 并将其返回值绑定到某个变量或用于其他表达式中。
 * 
 * 例如：
 * 
 * ``` rust
 * copy
 * fn main() {
 *     let x = another_function();  // 函数调用作为表达式执行
 *     println!("the value of x is: {}", x);
 * }
 * 
 * fn another_function() -> i32 {
 *     5  // 返回值作为表达式
 * }
 * ```
 * 在这个例子中, 函数another_function的返回值被赋值给变量x, 然后在println!宏中使用x的值。
 * 
 * 总结起来, rust中函数调用作为语句(即不需要返回值)执行时, 一般需要添加分号, 但可以省略分号, 用于执行某些副作用；而当函数调用作为表达式执行时, 不应该添加分号, 以便获取其返回值或将其用于其他表达式中。
 * 
 * 
 */
fn call_me() {
	print!("hola!")
}

fn main() {
    call_me();
}
