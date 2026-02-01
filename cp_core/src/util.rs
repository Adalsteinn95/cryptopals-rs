

/// Brute-force a single-byte XOR key over the given input bytes.
///
/// Returns the most likely plaintext as a String based on a simple score
/// that counts ASCII alphabetic characters and whitespace.
pub fn single_byte_xor_bruteforce(input_bytes: &[u8]) -> Option<String> {
	let mut best_score: usize = 0;
	let mut best_plaintext: Option<String> = None;

	for key in 0u8..=255u8 {
		let decrypted_bytes: Vec<u8> = input_bytes
			.iter()
			.map(|&b| b ^ key)
			.collect();

		if let Ok(decrypted_str) = String::from_utf8(decrypted_bytes.clone()) {
			let score = decrypted_str
				.chars()
				.filter(|c| c.is_ascii_alphabetic() || c.is_ascii_whitespace())
				.count();

			println!("Decrypted: {} | Score: {}", decrypted_str, score);
			if score > best_score {
				best_score = score;
				best_plaintext = Some(decrypted_str);
			}
		}
	}

	best_plaintext
}

	use anyhow::Result;

	/// Compute the Hamming distance in bits between two equal-length byte slices.
	/// Returns the number of differing bits.
	pub fn hamming_distance_bits(a: &[u8], b: &[u8]) -> Result<usize> {
		if a.len() != b.len() {
			anyhow::bail!("length mismatch: {} vs {}", a.len(), b.len());
		}

		let distance = a
			.iter()
			.zip(b.iter())
			.map(|(&x, &y)| (x ^ y).count_ones() as usize)
			.sum();

		Ok(distance)
	}