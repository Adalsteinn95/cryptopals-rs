
#[test]
fn set1_challenge5_repeating_key_xor() -> anyhow::Result<()> {

    let plaintext = "Burning 'em, if you ain't quick and nimble\n\
                     I go crazy when I hear a cymbal";

    let key = b"ICE";

    let plaintext_bytes = plaintext.as_bytes();

    let ciphertext: Vec<u8> = plaintext_bytes
        .iter()
        .enumerate()
        .map(|(i, &b)| b ^ key[i % key.len()])
        .collect();

    let expected_hex = "\
        0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
        a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

    let out_hex = hex::encode(ciphertext);
    assert_eq!(out_hex, expected_hex);

    Ok(())
}