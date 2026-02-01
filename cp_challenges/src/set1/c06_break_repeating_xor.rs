
use anyhow::Result;
use cp_core::encoding::{b64_to_bytes, bytes_to_utf8};
use cp_core::file::import_txt;
use cp_core::util::hamming_distance_bits;

fn normalized_edit_distance_for_keysize(bytes: &[u8], keysize: usize) -> Result<f64> {
    // Use first 4 blocks of length `keysize` and average all pairwise distances
    // for a more stable estimate.
    let blocks = 4;
    let needed = keysize * blocks;
    if bytes.len() < needed {
        anyhow::bail!("insufficient data for keysize {}: have {}, need {}", keysize, bytes.len(), needed);
    }

    let mut distances: Vec<f64> = Vec::new();
    for i in 0..blocks {
        let a = &bytes[(i * keysize)..((i + 1) * keysize)];
        for j in (i + 1)..blocks {
            let b = &bytes[(j * keysize)..((j + 1) * keysize)];
            let d = hamming_distance_bits(a, b)? as f64;
            distances.push(d / keysize as f64);
        }
    }

    Ok(distances.iter().copied().sum::<f64>() / distances.len() as f64)
}

fn guess_best_keysizes(bytes: &[u8], range: std::ops::RangeInclusive<usize>, top_n: usize) -> Vec<(usize, f64)> {
    let mut scored: Vec<(usize, f64)> = Vec::new();
    for k in range {
        if let Ok(score) = normalized_edit_distance_for_keysize(bytes, k) {
            scored.push((k, score));
        }
    }
    scored.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    scored.into_iter().take(top_n).collect()
}

fn transpose_blocks(bytes: &[u8], keysize: usize) -> Vec<Vec<u8>> {
    let mut cols: Vec<Vec<u8>> = vec![Vec::new(); keysize];
    for (i, &b) in bytes.iter().enumerate() {
        cols[i % keysize].push(b);
    }
    cols
}

fn score_english(s: &str) -> usize {
    s.chars().filter(|c| c.is_ascii_alphabetic() || c.is_ascii_whitespace()).count()
}

fn best_single_byte_xor_key(column: &[u8]) -> u8 {
    let mut best_key = 0u8;
    let mut best_score = 0usize;
    for key in 0u8..=255u8 {
        let decrypted: Vec<u8> = column.iter().map(|&b| b ^ key).collect();
        if let Ok(s) = String::from_utf8(decrypted) {
            let score = score_english(&s);
            if score > best_score {
                best_score = score;
                best_key = key;
            }
        }
    }
    best_key
}

fn repeating_key_xor_decrypt(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    ciphertext
        .iter()
        .enumerate()
        .map(|(i, &b)| b ^ key[i % key.len()])
        .collect()
}

#[test]
fn set1_challenge6_break_repeating_key_xor() -> Result<()> {
    // Read base64-encoded ciphertext from challenge file.
    let b64 = import_txt(concat!(env!("CARGO_MANIFEST_DIR"), "/src/set1/6.txt"))?;
    let ciphertext = b64_to_bytes(&b64)?;

    // Guess likely key sizes by normalized Hamming distance.
    let candidates = guess_best_keysizes(&ciphertext, 2..=40, 3);
    println!("Top keysize candidates: {:?}", candidates);

    // Try the best candidate first.
    let (best_keysize, _score) = candidates[0];
    let columns = transpose_blocks(&ciphertext, best_keysize);
    let key_bytes: Vec<u8> = columns.iter().map(|col| best_single_byte_xor_key(col)).collect();
    let key_str = String::from_utf8(key_bytes.clone()).unwrap_or_default();
    println!("Recovered key: {}", key_str);

    // Known key from the challenge description.
    assert_eq!(key_str, "Terminator X: Bring the noise");

    // Decrypt and verify we have readable plaintext.
    let plaintext_bytes = repeating_key_xor_decrypt(&ciphertext, &key_bytes);
    let plaintext = bytes_to_utf8(&plaintext_bytes)?;
    assert!(!plaintext.is_empty());

    Ok(())
}

#[test]
fn hamming_distance_example() -> anyhow::Result<()> {
    let a = b"this is a test";
    let b = b"wokka wokka!!!";
    let distance = hamming_distance_bits(a, b)?;
    assert_eq!(distance, 37);
    Ok(())
}
