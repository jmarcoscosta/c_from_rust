include!(concat!(env!("OUT_DIR"), "/binding.rs"));

fn main() {
    unsafe {
        let x = fibonacci(3);
        println!("{}", x);
    }
}
