#[link(name = "sum", kind = "static")]
extern "C" {
    pub fn sum(a: i32, b: i32) -> i32;
}

fn main() {
    let a = 3;
    let b = 4;
    let c = unsafe { sum(a, b) };
    println!("{} + {} = {}", a, b, c);
}
