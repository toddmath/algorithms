//! Simple XOR cipher.

/// XOR cipher algorithm.
#[inline]
pub fn xor_bytes(text: &[u8], key: u8) -> Vec<u8> {
    text.iter().map(|&b| b ^ key).collect()
}

/// XOR cipher algorithm.
#[inline]
pub fn xor(text: impl AsRef<str>, key: u8) -> Vec<u8> {
    xor_bytes(text.as_ref().as_bytes(), key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let key = 32;
        let text = "text string";
        let ciphered = xor(text, key);
        assert_eq!(text.as_bytes(), xor_bytes(&ciphered, key));
    }

    #[test]
    fn every_alphabetic_character_with_space() {
        let key = 64;
        let text = "The quick brown fox jumps over the lazy dog";
        let chiphered = xor(text, key);
        assert_eq!(text.as_bytes(), xor_bytes(&chiphered, key));
    }
}
