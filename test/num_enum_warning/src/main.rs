use num_enum::IntoPrimitive;

#[allow(unused)]
#[derive(IntoPrimitive)]
#[repr(u8)]
enum Example {
    Zero, One, Two
}

fn main() {
    println!("Hello, world!");
}
