use anyhow::Result;
use cp_core::encoding::{hex_to_bytes};

#[test]
fn set1_challenge2_fixed_xor() -> Result<()> {
    let input_hex1 = "1c0111001f010100061a024b53535009181c";
    let input_hex2 = "686974207468652062756c6c277320657965";
    let expected_hex = "746865206b696420646f6e277420706c6179";

    let bytes1 = hex_to_bytes(input_hex1)?;
    let bytes2 = hex_to_bytes(input_hex2)?;

    let xored: Vec<u8> = bytes1
        .iter()
        .zip(bytes2.iter())
        .map(|(&b1, &b2)| b1 ^ b2)
        .collect();


    let out_hex = hex::encode(xored);
    assert_eq!(out_hex, expected_hex);
    Ok(())

}