//! It autocomplete by prefix using added words.
//!
//! word List => ["apple", "orange", "oregano"]
//! prefix => "or"
//! matches => ["orange", "oregano"]

use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use itertools::Itertools;

const END: char = '#';

#[derive(Debug)]
struct Trie(HashMap<char, Box<Trie>>);

impl Deref for Trie {
    type Target = HashMap<char, Box<Trie>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Trie {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Trie {
    #[inline]
    fn new() -> Self {
        Trie(HashMap::new())
    }

    fn insert(&mut self, text: impl AsRef<str>) {
        let mut trie = self;
        let text = text.as_ref();

        for c in text.chars() {
            trie = trie.entry(c).or_insert_with(|| Box::new(Trie::new()));
        }

        trie.deref_mut().insert(END, Box::new(Trie::new()));
    }

    fn find(&self, prefix: impl AsRef<str>) -> Vec<String> {
        let mut trie = self;
        let prefix = prefix.as_ref();

        for c in prefix.chars() {
            match trie.get(&c) {
                Some(char_trie) => trie = char_trie,
                None => return vec![],
            }
        }

        Self::elements(trie)
            .iter()
            .map(|s| format!("{prefix}{s}"))
            .collect_vec()
    }

    fn elements(map: &Trie) -> Vec<String> {
        map.iter()
            .flat_map(|(c, v)| {
                if c == &END {
                    vec!["".to_string()]
                } else {
                    Self::elements(v)
                        .iter()
                        .map(|s| format!("{c}{s}"))
                        .collect_vec()
                }
            })
            .collect_vec()
    }
}

/// Autocomplete data structure based on a [`Trie`].
#[derive(Debug)]
pub struct Autocomplete {
    trie: Trie,
}

impl Default for Autocomplete {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Autocomplete {
    /// Creates a new [`Autocomplete`].
    #[inline]
    pub fn new() -> Self {
        Self { trie: Trie::new() }
    }

    /// Insert words into the [`Autocomplete`].
    pub fn insert_words(&mut self, words: Vec<String>) {
        for word in words {
            self.trie.insert(word);
        }
    }

    /// Find words in the [`Autocomplete`] by prefix.
    pub fn find_words(&self, prefix: impl AsRef<str>) -> Vec<String> {
        self.trie.find(prefix.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::Autocomplete;

    #[test]
    fn test_autocomplete() {
        let words = vec![
            "apple".to_owned(),
            "orange".to_owned(),
            "oregano".to_owned(),
        ];

        let mut auto_complete = Autocomplete::new();
        auto_complete.insert_words(words);

        let prefix = "app".to_owned();
        let mut auto_completed_words = auto_complete.find_words(prefix);

        let mut apple = vec!["apple".to_owned()];
        apple.sort();

        auto_completed_words.sort();
        assert_eq!(auto_completed_words, apple);

        let prefix = "or".to_owned();
        let mut auto_completed_words = auto_complete.find_words(prefix);

        let mut prefix_or = vec!["orange".to_owned(), "oregano".to_owned()];
        prefix_or.sort();

        auto_completed_words.sort();
        assert_eq!(auto_completed_words, prefix_or);
    }
}
