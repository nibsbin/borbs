fn main() {
    println!("Hello, world!");
    wrapper(-5)
}

fn wrapper (x:i32) {
    let wrapped = {
        another_func(x+7)
    };
    wrapped
}

fn another_func(x:i32) {
    println!("hello from another fun. ur value is {x}
    ")
}