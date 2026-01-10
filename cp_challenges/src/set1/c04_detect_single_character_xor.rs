use cp_core::file::import_txt;
use cp_core::encoding::hex_to_bytes;


#[test]
fn set1_challenge4_detect_single_character_xor() -> anyhow::Result<()> {
    let lines = import_txt(concat!(env!("CARGO_MANIFEST_DIR"), "/src/set1/4.txt"))?;
    let mut best_score = 0;
    let mut best_plaintext = String::new();

    for line in lines.lines() {
        let input_bytes = hex_to_bytes(line)?;

        if let Some(plaintext) = cp_core::util::single_byte_xor_bruteforce(&input_bytes) {
            let score = plaintext.chars().filter(|c| c.is_ascii_alphabetic() || c.is_ascii_whitespace()).count();
            if score > best_score {
                best_score = score;
                best_plaintext = plaintext;
            }
        }
    }
    println!("Best plaintext: {}", best_plaintext);
    println!("Best score: {}", best_score);
    Ok(())
}