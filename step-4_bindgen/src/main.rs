#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    let a = 2;
    let b = 3;
    println!("Add {a} and {b}, result is {}", unsafe { add(2, 3) });
}
