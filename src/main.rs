include!(concat!(env!("OUT_DIR"), "/binding.rs"));

fn main() {
    unsafe {
        for n in 0..10 {
            println!("{}", fibonacci(n));
        }
    }
}
