fn main() {
    println!("control-c to exit");
    let mut i = 0;
    loop {
        println!("Hello, world! {0}", i);
        i += 1;
    }
}
