use anyhow::{bail, Result};

pub fn hex_to_bytes(s: &str) -> Result<Vec<u8>> {
    let s = s.trim();
    Ok(hex::decode(s)?)
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

pub fn b64_to_bytes(s: &str) -> Result<Vec<u8>> {
    let s = s.trim();
    let engine = base64::engine::general_purpose::STANDARD;
    Ok(base64::Engine::decode(&engine, s)?)
}

pub fn bytes_to_b64(bytes: &[u8]) -> String {
    let engine = base64::engine::general_purpose::STANDARD;
    base64::Engine::encode(&engine, bytes)
}

/// Strict UTF-8 conversion (Cryptopals often uses raw bytes—don't auto-fix).
pub fn bytes_to_utf8(bytes: &[u8]) -> Result<String> {
    Ok(std::str::from_utf8(bytes)?.to_string())
}

pub fn utf8_to_bytes(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
}

/// Useful guard when you're expecting equal lengths.
pub fn ensure_same_len(a: &[u8], b: &[u8]) -> Result<()> {
    if a.len() != b.len() {
        bail!("length mismatch: {} vs {}", a.len(), b.len());
    }
    Ok(())
}
