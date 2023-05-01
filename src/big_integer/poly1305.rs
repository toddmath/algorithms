//! Ply1305 Message Authentication Code
//!
//! This implementation is based on RFC8439.
//! Note that the Big Integer library we are using may not be suitable for
//! cryptographic applications due to non constant time operations.

use num_bigint::BigUint;
use num_traits::{Num, Zero};

#[doc(hidden)]
macro_rules! hex_uint {
    ($a:literal) => {
        BigUint::from_str_radix($a, 16).unwrap()
    };
}

/// Ply1305 Message Authentication Code
///
/// This implementation is based on RFC8439.
/// Note that the Big Integer library we are using may not be suitable for
/// cryptographic applications due to non constant time operations.
#[derive(Debug)]
pub struct Poly1305 {
    p: BigUint,
    r: BigUint,
    s: BigUint,
    /// The accumulator
    pub acc: BigUint,
}

impl Default for Poly1305 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Poly1305 {
    #[allow(missing_docs)]
    #[inline]
    pub fn new() -> Self {
        Self {
            p: hex_uint!("3fffffffffffffffffffffffffffffffb"), // 2^130 - 5
            r: BigUint::zero(),
            s: BigUint::zero(),
            acc: BigUint::zero(),
        }
    }

    #[allow(missing_docs)]
    pub fn clamp_r(&mut self) {
        self.r &= hex_uint!("0ffffffc0ffffffc0ffffffc0fffffff");
    }

    #[allow(missing_docs)]
    pub fn set_key(&mut self, key: &[u8; 32]) {
        self.r = BigUint::from_bytes_le(&key[..16]);
        self.s = BigUint::from_bytes_le(&key[16..]);
        self.clamp_r();
    }

    /// process a 16-byte-long message block. If message is not long enough,
    /// fill the `msg` array with zeros, but set `msg_bytes` to the original
    /// chunk length in bytes. See `basic_tv1` for example usage.
    pub fn add_msg(&mut self, msg: &[u8; 16], msg_bytes: u64) {
        let mut n = BigUint::from_bytes_le(msg);
        n.set_bit(msg_bytes * 8, true);
        self.acc += n;
        self.acc *= &self.r;
        self.acc %= &self.p;
    }

    /// The result is guaranteed to be 16 bytes long
    pub fn get_tag(&self) -> Vec<u8> {
        let result = &self.acc + &self.s;
        let mut bytes = result.to_bytes_le();
        bytes.resize(16, 0);
        bytes
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;

    use super::*;

    fn get_tag_hex(tag: &[u8]) -> String {
        let mut result = String::new();
        for &x in tag {
            write!(result, "{x:02x}").unwrap();
        }
        result
    }

    #[test]
    fn basic_tv1() {
        let mut mac = Poly1305::new();
        let key: [u8; 32] = [
            0x85, 0xd6, 0xbe, 0x78, 0x57, 0x55, 0x6d, 0x33, 0x7f, 0x44, 0x52, 0xfe, 0x42, 0xd5,
            0x06, 0xa8, 0x01, 0x03, 0x80, 0x8a, 0xfb, 0x0d, 0xb2, 0xfd, 0x4a, 0xbf, 0xf6, 0xaf,
            0x41, 0x49, 0xf5, 0x1b,
        ];
        let mut tmp_buffer = [0_u8; 16];
        mac.set_key(&key);
        mac.add_msg(b"Cryptographic Fo", 16);
        mac.add_msg(b"rum Research Gro", 16);
        tmp_buffer[..2].copy_from_slice(b"up");
        mac.add_msg(&tmp_buffer, 2);

        let result = mac.get_tag();

        assert_eq!(
            get_tag_hex(result.as_slice()),
            "a8061dc1305136c6c22b8baf0c0127a9"
        );
    }
}
