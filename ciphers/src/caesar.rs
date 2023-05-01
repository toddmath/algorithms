//! Caesar Cipher
//! Based on cipher_crypt::caesar
//!
//! # Algorithm
//!
//! Rotate each ascii character by shift. The most basic example is ROT 13,
//! which rotates 'a' to 'n'. This implementation does not rotate unicode
//! characters.

/// Caesar cipher to shif, or rotate, cipher text.
pub fn caesar(cipher: impl AsRef<str>, shift: u8) -> String {
    cipher
        .as_ref()
        .chars()
        .map(|c| match c {
            'A'..='Z' => char::from(b'A' + (c as u8 + shift - b'A') % 26),
            'a'..='z' => char::from(b'a' + (c as u8 + shift - b'a') % 26),
            _ => c,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(caesar("", 13), "");
    }

    #[test]
    fn caesar_rot_13() {
        assert_eq!(caesar("rust", 13), "ehfg");
    }

    #[test]
    fn caesar_unicode() {
        assert_eq!(caesar("attack at dawn 攻", 5), "fyyfhp fy ifbs 攻");
    }
}
