mod input;
mod one;

use crate::{input::INSTRUCTION_INTPUT, one::decode_password_v2};

fn main() {
    let zero_count = decode_password_v2(INSTRUCTION_INTPUT);
    println!("final: {}", zero_count);
}
