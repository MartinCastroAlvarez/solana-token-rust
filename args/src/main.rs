use std::env::{args, Args};

fn main() {
    let mut args: Args = args();
    let first: String = args.nth(1).unwrap();
    let second: String = args.nth(0).unwrap();
    let third: String = args.nth(0).unwrap();
    println!("Hello, {:?} {:?} {:?}!", first, second, third);
}
