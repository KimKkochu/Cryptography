pub fn encryption(plaintext: &i32, key: &i32) -> i32 {
	plaintext ^ key
}

pub fn decryptrion(ciphertext: &i32, key: &i32) -> i32 {
	ciphertext ^ key
}