use anyhow::Result;
use cp_core::encoding::{bytes_to_b64, hex_to_bytes};


#[test]
fn set1_challenge1_hex_to_base64() -> Result<()> {
    let input_hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected_b64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    let bytes = hex_to_bytes(input_hex)?;
    let out = bytes_to_b64(&bytes);

    assert_eq!(out, expected_b64);
    Ok(())
}
