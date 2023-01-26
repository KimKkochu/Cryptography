#![allow(non_snake_case)]

use Cryptography::OTP;

fn main() {
    let key: i32 = 0b10110100;
    let plaintext: i32 = 0b01101101;
    let ciphertext = OTP::encryption(&plaintext, &key);
    let decodetext = OTP::decryptrion(&ciphertext, &key);
    
    println!("Ciphertext: {:b}, decodetext: {:b}", ciphertext, decodetext);
}
