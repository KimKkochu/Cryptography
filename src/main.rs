#![allow(non_snake_case)]

use Cryptography::OTP::encryption;

fn main() {
    let i = encryption(0b01101101, 0b10110100);
    println!("{:b}",i);
}
