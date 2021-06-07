mod binding;
use binding::fibonacci;

fn main() {
    unsafe {
        let x = fibonacci(3);
        println!("{}", x);
    }
}
