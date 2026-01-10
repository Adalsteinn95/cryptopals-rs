use anyhow::Result;
use cp_core::encoding::hex_to_bytes;
use cp_core::util::single_byte_xor_bruteforce;


#[test]
fn set1_challenge3_single_byte_xor_cipher() -> Result<()> {
    // hex encoded string 
    let input_hex = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    // has been xored against a single character and we need to find the key by brute forcing
    let input_bytes = hex_to_bytes(input_hex)?;
    let mut best_score = 0;
    let mut best_plaintext = String::new();

    // remember xoring the key twice cancels out 
    // plaintext ^ key = ciphertext
    // ciphertext ^ key = plaintext


    if let Some(best_plaintext) = single_byte_xor_bruteforce(&input_bytes) {
        println!("Best plaintext: {}", best_plaintext);
    } else {
        println!("No valid plaintext found.");
    }

    Ok(())



}