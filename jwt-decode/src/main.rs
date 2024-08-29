#![no_std]
#![no_main]

#[no_mangle]
fn main() -> i32 {
    decode_lib::run()
}
