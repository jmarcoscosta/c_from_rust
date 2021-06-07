extern crate cc;

fn main() {
    println!("calling build.rs");
    cc::Build::new()
        .file("src/fibonacci.c")
        .compile("libfibonacci.a");
}
