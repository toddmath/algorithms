//! The common implementation of a CountMinSketch
//! Holding a DEPTH x WIDTH matrix of counts
//!
//! The idea behind the implementation is the following:
//! Let's start from our problem statement above. We have a frequency map of
//! counts, and want to go reduce its space complexity The immediate way to do
//! this would be to use a Vector with a fixed size, let this size be `WIDTH` We
//! will be holding the count of each item `item` in the Vector, at index `i =
//! hash(item) % WIDTH` where `hash` is a hash function: `item -> usize`
//! We now have constant space.
//!
//! The problem though is that we'll potentially run into a lot of collisions.
//! Taking an extreme example, if `WIDTH = 1`, all items will have the same
//! count, which is the sum of counts of every items We could reduce the amount
//! of collisions by using a bigger `WIDTH` but this wouldn't be way more
//! efficient than the "big" frequency map How do we improve the solution, but
//! still keeping constant space?
//!
//! The idea is to use not just one vector, but multiple (`DEPTH`) ones and
//! attach different `hash` functions to each vector This would lead to the
//! following data structure:             <- WIDTH = 5 ->
//!  D   hash1: [0, 0, 0, 0, 0]
//!  E   hash2: [0, 0, 0, 0, 0]
//!  P   hash3: [0, 0, 0, 0, 0]
//!  T   hash4: [0, 0, 0, 0, 0]
//!  H   hash5: [0, 0, 0, 0, 0]
//!  =   hash6: [0, 0, 0, 0, 0]
//!  7   hash7: [2, 0, 0, 0, 0]
//!
//! We actually can witness some collisions (invalid counts of `2` above in some
//! rows). This means that if we have to return the count for "TEST", we'd
//! actually fetch counts from every row and return the minimum value
//!
//! This could potentially be overestimated if we have a huge number of entries
//! and a lot of collisions. But an interesting property is that the count we
//! return for "TEST" cannot be underestimated

use std::{
    array,
    collections::hash_map::RandomState,
    fmt::{Debug, Formatter},
    hash::{BuildHasher, Hash, Hasher},
    marker::PhantomData,
};

/// A probabilistic data structure holding an approximate count for diverse
/// items efficiently (using constant space)
///
/// Let's imagine we want to count items from an incoming (unbounded) data
/// stream One way to do this would be to hold a frequency hashmap, counting
/// element hashes This works extremely well, but unfortunately would require a
/// lot of memory if we have a huge diversity of incoming items in the data
/// stream
///
/// CountMinSketch aims at solving this problem, trading off the exact count for
/// an approximate one, but getting from potentially unbounded space complexity
/// to constant complexity See the implementation below for more details
///
/// Here is the definition of the different allowed operations on a
/// CountMinSketch:
///     * increment the count of an item
///     * retrieve the count of an item
pub trait CountMinSketch {
    /// The type of the items to count.
    type Item;

    /// Increment the count of an item.
    fn increment(&mut self, item: Self::Item);

    /// Increment the count of an item by a given amount.
    fn increment_by(&mut self, item: Self::Item, count: usize);

    /// Get the count of an item.
    fn get_count(&self, item: Self::Item) -> usize;
}

/// The common implementation of a CountMinSketch
/// Holding a DEPTH x WIDTH matrix of counts
///
/// The idea behind the implementation is the following:
/// Let's start from our problem statement above. We have a frequency map of
/// counts, and want to go reduce its space complexity The immediate way to do
/// this would be to use a Vector with a fixed size, let this size be `WIDTH` We
/// will be holding the count of each item `item` in the Vector, at index `i =
/// hash(item) % WIDTH` where `hash` is a hash function: `item -> usize`
/// We now have constant space.
///
/// The problem though is that we'll potentially run into a lot of collisions.
/// Taking an extreme example, if `WIDTH = 1`, all items will have the same
/// count, which is the sum of counts of every items We could reduce the amount
/// of collisions by using a bigger `WIDTH` but this wouldn't be way more
/// efficient than the "big" frequency map How do we improve the solution, but
/// still keeping constant space?
///
/// The idea is to use not just one vector, but multiple (`DEPTH`) ones and
/// attach different `hash` functions to each vector This would lead to the
/// following data structure:             <- WIDTH = 5 ->
///  D   hash1: [0, 0, 0, 0, 0]
///  E   hash2: [0, 0, 0, 0, 0]
///  P   hash3: [0, 0, 0, 0, 0]
///  T   hash4: [0, 0, 0, 0, 0]
///  H   hash5: [0, 0, 0, 0, 0]
///  =   hash6: [0, 0, 0, 0, 0]
///  7   hash7: [2, 0, 0, 0, 0]
///
/// We actually can witness some collisions (invalid counts of `2` above in some
/// rows). This means that if we have to return the count for "TEST", we'd
/// actually fetch counts from every row and return the minimum value
///
/// This could potentially be overestimated if we have a huge number of entries
/// and a lot of collisions. But an interesting property is that the count we
/// return for "TEST" cannot be underestimated
pub struct HashCountMinSketch<Item: Hash, const WIDTH: usize, const DEPTH: usize> {
    phantom: PhantomData<Item>, // just a marker for Item to be used
    counts: [[usize; WIDTH]; DEPTH],
    hashers: [RandomState; DEPTH],
}

impl<Item: Hash, const WIDTH: usize, const DEPTH: usize> Debug
    for HashCountMinSketch<Item, WIDTH, DEPTH>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Item").field("vecs", &self.counts).finish()
    }
}

impl<T: Hash, const WIDTH: usize, const DEPTH: usize> Default
    for HashCountMinSketch<T, WIDTH, DEPTH>
{
    fn default() -> Self {
        let hashers = array::from_fn::<RandomState, DEPTH, _>(|_| RandomState::new());
        Self {
            phantom: Default::default(),
            counts: [[0; WIDTH]; DEPTH],
            hashers,
        }
    }
}

impl<Item: Hash, const WIDTH: usize, const DEPTH: usize> CountMinSketch
    for HashCountMinSketch<Item, WIDTH, DEPTH>
{
    type Item = Item;

    fn increment(&mut self, item: Self::Item) {
        self.increment_by(item, 1)
    }

    fn increment_by(&mut self, item: Self::Item, count: usize) {
        for (row, r) in self.hashers.iter_mut().enumerate() {
            let mut hasher = r.build_hasher();
            item.hash(&mut hasher);
            let hashed = hasher.finish();
            self.counts[row][(hashed % WIDTH as u64) as usize] += count;
        }
    }

    fn get_count(&self, item: Self::Item) -> usize {
        self.hashers
            .iter()
            .enumerate()
            .map(|(row, r)| {
                let mut h = r.build_hasher();
                item.hash(&mut h);
                let hashed = h.finish();
                self.counts[row][(hashed % WIDTH as u64) as usize]
            })
            .min()
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use quickcheck::{Arbitrary, Gen};

    use super::{CountMinSketch, HashCountMinSketch};

    #[test]
    fn hash_functions_should_hash_differently() {
        let mut sketch: HashCountMinSketch<&str, 50, 50> = HashCountMinSketch::default(); // use a big DEPTH
        sketch.increment("something");
        // We want to check that our hash functions actually produce different results,
        // so we'll store the indices where we encounter a count=1 in a set
        let mut indices_of_ones: HashSet<usize> = HashSet::default();
        for counts in sketch.counts {
            let ones = counts
                .into_iter()
                .enumerate()
                .filter_map(|(idx, count)| (count == 1).then_some(idx))
                .collect::<Vec<_>>();
            assert_eq!(1, ones.len());
            indices_of_ones.insert(ones[0]);
        }
        // Given the parameters (WIDTH = 50, DEPTH = 50) it's extremely unlikely that
        // all hash functions hash to the same index
        assert!(indices_of_ones.len() > 1); // but we want to avoid a bug where all hash functions would produce the same hash (or hash to the same index)
    }

    #[test]
    fn inspect_counts() {
        let mut sketch: HashCountMinSketch<&str, 5, 7> = HashCountMinSketch::default();
        sketch.increment("test");
        // Inspect internal state:
        for counts in sketch.counts {
            let zeroes = counts.iter().filter(|count| **count == 0).count();
            assert_eq!(4, zeroes);
            let ones = counts.iter().filter(|count| **count == 1).count();
            assert_eq!(1, ones);
        }
        sketch.increment("test");
        for counts in sketch.counts {
            let zeroes = counts.iter().filter(|count| **count == 0).count();
            assert_eq!(4, zeroes);
            let twos = counts.iter().filter(|count| **count == 2).count();
            assert_eq!(1, twos);
        }

        // This one is actually deterministic
        assert_eq!(2, sketch.get_count("test"));
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    struct TestItem {
        item: String,
        count: usize,
    }

    const MAX_STR_LEN: u8 = 30;
    const MAX_COUNT: usize = 20;

    impl Arbitrary for TestItem {
        fn arbitrary(g: &mut Gen) -> Self {
            let str_len = u8::arbitrary(g) % MAX_STR_LEN;
            let mut str = String::with_capacity(str_len as usize);
            for _ in 0..str_len {
                str.push(char::arbitrary(g));
            }
            let count = usize::arbitrary(g) % MAX_COUNT;
            TestItem { item: str, count }
        }
    }

    #[quickcheck_macros::quickcheck]
    fn must_not_understimate_count(test_items: Vec<TestItem>) {
        let test_items = test_items.into_iter().collect::<HashSet<_>>(); // remove duplicated (would lead to weird counts)
        let n = test_items.len();
        let mut sketch: HashCountMinSketch<String, 50, 10> = HashCountMinSketch::default();
        let mut exact_count = 0;
        for TestItem { item, count } in &test_items {
            sketch.increment_by(item.clone(), *count);
        }
        for TestItem { item, count } in test_items {
            let stored_count = sketch.get_count(item);
            assert!(stored_count >= count);
            if count == stored_count {
                exact_count += 1;
            }
        }
        if n > 20 {
            // if n is too short, the stat isn't really relevant
            let exact_ratio = exact_count as f64 / n as f64;
            assert!(exact_ratio > 0.7); // the proof is quite hard, but this should be OK
        }
    }
}
