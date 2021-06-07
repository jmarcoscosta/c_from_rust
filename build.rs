extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/fibonacci.c")
        .compile("libfibonacci.a");
}
