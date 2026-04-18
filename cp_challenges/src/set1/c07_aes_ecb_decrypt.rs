use anyhow::Result;
use aes::Aes128;
use aes::cipher::{BlockCipherDecrypt, KeyInit};
use cp_core::encoding::{b64_to_bytes, bytes_to_utf8};
use cp_core::file::import_txt;

/// Decrypt AES-128 in ECB mode.
///
/// ECB ("Electronic Codebook") simply applies the block cipher to each 16-byte
/// block independently.  There is no chaining, no IV, no nonce — every block is
/// an independent AES decryption with the same key.  That independence is what
/// makes ECB insecure for real data (identical plaintext blocks produce identical
/// ciphertext blocks), but it makes implementation trivial: iterate over 16-byte
/// chunks and call `decrypt_block` on each.
pub fn aes_ecb_decrypt(ciphertext: &[u8], key: &[u8; 16]) -> Result<Vec<u8>> {
    if ciphertext.len() % 16 != 0 {
        anyhow::bail!("ciphertext length {} is not a multiple of 16", ciphertext.len());
    }

    let cipher = Aes128::new_from_slice(key)?;
    let mut plaintext = ciphertext.to_vec();

    for chunk in plaintext.chunks_mut(16) {
        let block: &mut aes::Block = chunk.try_into().expect("chunk is exactly 16 bytes");
        cipher.decrypt_block(block);
    }

    Ok(plaintext)
}

#[test]
fn set1_challenge7_aes_ecb_decrypt() -> Result<()> {
    let b64 = import_txt(concat!(env!("CARGO_MANIFEST_DIR"), "/src/set1/7.txt"))?;
    let ciphertext = b64_to_bytes(&b64)?;

    let key: [u8; 16] = *b"YELLOW SUBMARINE";
    let plaintext_bytes = aes_ecb_decrypt(&ciphertext, &key)?;
    let plaintext = bytes_to_utf8(&plaintext_bytes)?;

    // The decrypted text starts with a known phrase from the challenge.
    assert!(plaintext.starts_with("I'm back and I'm ringin' the bell"));
    println!("Decrypted {} bytes. First line:\n{}", plaintext.len(), plaintext.lines().next().unwrap_or(""));

    Ok(())
}
