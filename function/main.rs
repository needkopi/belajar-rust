/*
 *
 * ini adalah seri fungsi (function)
 *
 * di rust deklarasi fungsi dengan keyword `fn`
 *
 * */

/*
 * fungsi main adalah fungsi yang pertama kali dijalankan
 *
 * */
fn main() {
    another_function();
    param_func(5);

    let x = return_func(5);
    println!("value of x is: {}",x)
}

/*
 * by default rust use snake case typing
 * */
fn another_function() {
    println!("from another function");
}

/*
 * fungsi dengan parameter
 * */
fn param_func(x: i32) {
    println!("value of parameter is: {}",x);
}

/*
 * fungsi dengan return value
 * */
fn return_func(x: i32) -> i32 {
    return x;
}
