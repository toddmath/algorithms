use std::{
    cmp::Ordering,
    collections::{BTreeMap, BinaryHeap},
};

/// Value for a [`HuffmanDictionary`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HuffmanValue {
    /// The encoded value
    pub value: u64,
    /// number of bits used (up to 64)
    pub bits: u32,
}

/// Node for a [`HuffmanDictionary`].
#[derive(Debug, Default, Clone)]
pub struct HuffmanNode<T> {
    /// left child
    pub left: Option<Box<HuffmanNode<T>>>,
    /// right child
    pub right: Option<Box<HuffmanNode<T>>>,
    /// symbol
    pub symbol: Option<T>,
    /// frequency
    pub frequency: u64,
}

impl<T> PartialEq for HuffmanNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}

impl<T> Eq for HuffmanNode<T> {}

impl<T> PartialOrd for HuffmanNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.frequency.cmp(&other.frequency).reverse())
    }
}

impl<T> Ord for HuffmanNode<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency.cmp(&other.frequency).reverse()
    }
}

impl<T: Copy + Ord> HuffmanNode<T> {
    /// Creates a new [`HuffmanNode<T>`].
    #[inline]
    pub fn new(symbol: T, frequency: u64) -> Self {
        HuffmanNode {
            left: None,
            right: None,
            symbol: Some(symbol),
            frequency,
        }
    }

    /// Creates a new [`HuffmanNode<T>`] with `left` and `right` children.
    #[inline]
    pub fn with_children(left: Self, right: Self, frequency: u64) -> Self {
        HuffmanNode {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            symbol: None,
            frequency,
        }
    }

    /// Turn the tree into the map that can be used in encoding
    pub fn get_alphabet(
        height: u32,
        path: u64,
        node: &HuffmanNode<T>,
        map: &mut BTreeMap<T, HuffmanValue>,
    ) {
        match node.symbol {
            Some(s) => {
                map.insert(s, HuffmanValue {
                    value: path,
                    bits: height,
                });
            }
            None => {
                Self::get_alphabet(height + 1, path, node.left.as_ref().unwrap(), map);
                Self::get_alphabet(
                    height + 1,
                    path | (1 << height),
                    node.right.as_ref().unwrap(),
                    map,
                );
            }
        }
    }
}

/// Dictionary for [`HuffmanEncoding`].
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HuffmanDictionary<T> {
    /// The alphabet
    pub alphabet: BTreeMap<T, HuffmanValue>,
    /// The root of the Huffman tree
    pub root: HuffmanNode<T>,
}

impl<T: Copy + Ord> HuffmanDictionary<T> {
    /// The list of alphabet symbols and their respective frequency should
    /// be given as input
    pub fn new(alphabet: &[(T, u64)]) -> Self {
        let mut alpha = BTreeMap::new();

        let mut queue = alphabet
            .iter()
            .map(|&(symbol, freq)| HuffmanNode::new(symbol, freq))
            .collect::<BinaryHeap<_>>();

        for _ in 1..alphabet.len() {
            let left = queue.pop().unwrap();
            let right = queue.pop().unwrap();
            let sm_freq = left.frequency + right.frequency;
            queue.push(HuffmanNode::with_children(left, right, sm_freq));
        }

        let root = queue.pop().unwrap();
        HuffmanNode::get_alphabet(0, 0, &root, &mut alpha);

        HuffmanDictionary {
            alphabet: alpha,
            root,
        }
    }

    /// Encode `data` using the dictionary
    pub fn encode(&self, data: &[T]) -> HuffmanEncoding {
        let mut result = HuffmanEncoding::new();
        for value in data {
            result.add_data(*self.alphabet.get(value).unwrap());
        }
        result
    }
}

/// Encoding for [`HuffmanDictionary`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HuffmanEncoding {
    /// Number of bits used
    pub num_bits: u64,
    /// Encoded data
    pub data: Vec<u64>,
}

impl Default for HuffmanEncoding {
    #[inline]
    fn default() -> Self {
        HuffmanEncoding {
            num_bits: 0,
            data: vec![0],
        }
    }
}

impl HuffmanEncoding {
    /// Creates a new [`HuffmanEncoding`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds `data` to the encoding.
    ///
    /// # Panics
    ///
    /// Panics if `data` is empty.
    #[inline]
    pub fn add_data(&mut self, data: HuffmanValue) {
        assert!(!self.data.is_empty());
        let shift = (self.num_bits & 63) as u32;
        let val = data.value;
        *self.data.last_mut().unwrap() |= val.wrapping_shl(shift);
        if (shift + data.bits) >= 64 {
            self.data.push(val.wrapping_shr(64 - shift));
        }
        self.num_bits += data.bits as u64;
    }

    /// In case the encoding is invalid, `None` is returned
    pub fn decode<T: Copy + Ord>(&self, dict: &HuffmanDictionary<T>) -> Option<Vec<T>> {
        let mut state = &dict.root;
        let mut result = Vec::with_capacity(self.num_bits as usize);

        for i in 0..self.num_bits {
            if let Some(symbol) = state.symbol {
                result.push(symbol);
                state = &dict.root;
            }
            state = if self.get_bit(i) {
                state.right.as_ref()?
            } else {
                state.left.as_ref()?
            };
        }

        if self.num_bits > 0 {
            result.push(state.symbol?);
        }
        Some(result)
    }

    #[inline]
    fn get_bit(&self, pos: u64) -> bool {
        (self.data[(pos >> 6) as usize] & (1 << (pos & 63))) != 0
    }
}

// cSPell: disable
#[cfg(test)]
mod tests {
    use super::*;

    fn get_frequency(bytes: &[u8]) -> Vec<(u8, u64)> {
        let mut cnts: Vec<u64> = vec![0; 256];
        for &b in bytes.iter() {
            cnts[b as usize] += 1
        }
        cnts.iter()
            .enumerate()
            .filter(|(_, &v)| v > 0)
            .map(|(b, &cnt)| (b as u8, cnt))
            .collect()
    }

    #[test]
    fn small_text() {
        let text = "Hello world";
        let bytes = text.as_bytes();
        let freq = get_frequency(bytes);
        let dict = HuffmanDictionary::new(&freq);
        let encoded = dict.encode(bytes);
        assert_eq!(encoded.num_bits, 32);

        let decoded = encoded.decode(&dict).unwrap();
        assert_eq!(decoded, bytes);
    }

    #[test]
    fn lorem_ipsum() {
        let text = concat!(
            "The quick brown fox jumped over the lazy dog.",
            "Lorem ipsum dolor sit amet, consectetur ",
            "adipiscing elit, sed do eiusmod tempor incididunt ut labore et ",
            "dolore magna aliqua. Facilisis magna etiam tempor orci. Nullam ",
            "non nisi est sit amet facilisis magna. Commodo nulla facilisi ",
            "nullam vehicula. Interdum posuere lorem ipsum dolor. Elit eget ",
            "gravida cum sociis natoque penatibus. Dictum sit amet justo donec ",
            "enim. Tempor commodo ullamcorper a lacus vestibulum sed. Nisl ",
            "suscipit adipiscing bibendum est ultricies. Sit amet aliquam id ",
            "diam maecenas ultricies."
        );
        let bytes = text.as_bytes();
        let freq = get_frequency(bytes);
        let dict = HuffmanDictionary::new(&freq);
        let encoded = dict.encode(bytes);
        assert_eq!(encoded.num_bits, 2372);

        let decoded = encoded.decode(&dict).unwrap();
        assert_eq!(decoded, bytes);

        let text = "The dictionary should work on other texts too";
        let bytes = text.as_bytes();
        let encoded = dict.encode(bytes);
        assert_eq!(encoded.num_bits, 215);

        let decoded = encoded.decode(&dict).unwrap();
        assert_eq!(decoded, bytes);
    }
}
