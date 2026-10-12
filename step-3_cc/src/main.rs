unsafe extern "C" {
    fn add(a: i64, b: i64) -> i64;
}

fn main() {
    let a = 2;
    let b = 3;
    println!("Add {a} and {b}, result is {}", unsafe { add(2, 3) });
}
