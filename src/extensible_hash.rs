// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

/// Fill-Degree Constraint
/// Hash table grows if a proprtion of buckets larger than this is filled.
const FDC: f32 = 0.8;

const ITEMS_PER_BUCKET: usize = 10;

struct HashTable<V> {
    buckets: Vec<Vec<(usize, V)>>,
    num_bits: usize,
    num_items: usize,
}

impl<V> HashTable<V> {
    pub fn new() -> HashTable<V> {
        return HashTable {
            buckets: vec![Vec::new()],
            num_bits: 1,
            num_items: 0,
        };
    }

    pub fn put(&mut self, k: usize, v: V) {
        let b = self.bucket(k);
        self.buckets[b].push((k, v));
        self.num_items += 1;

        if self.should_split() {
            self.split();
        }
    }

    pub fn get(&self, key: usize) -> Option<&V> {
        let b = self.bucket(key);
        for (k, v) in &self.buckets[b] {
            if *k == key {
                return Some(&v);
            }
        }
        return None;
    }

    /// This is an O(n) operation because of the remove from Vec.
    pub fn remove(&mut self, key: usize) {
        let b = self.bucket(key);
        for i in 0..self.buckets[b].len() {
            if self.buckets[b][i].0 == key {
                self.buckets[b].remove(i);
            }
            break;
        }
        self.num_items -= 1;
    }

    /// This is an O(n^2) operation because of the remove from Vec.
    fn split(&mut self) {
        if self.buckets.len() == (1 << self.num_bits) {
            self.num_bits += 1;
        }

        let b_split = self.buckets.len() - (1 << (self.num_bits - 1));
        self.buckets.push(Vec::new());
        let mut i = 0;
        while i != self.buckets[b_split].len() {
            if self.bucket(self.buckets[b_split][i].0) != b_split {
                let (k, v) = self.buckets[b_split].remove(i);
                self.put(k, v);
            } else {
                i += 1;
            }
        }
    }

    fn should_split(&self) -> bool {
        return self.num_items as f32 / (self.buckets.len() * ITEMS_PER_BUCKET) as f32 > FDC;
    }

    fn bucket(&self, k: usize) -> usize {
        let h = k % (1 << self.num_bits);
        if h >= self.buckets.len() {
            h - (1 << (self.num_bits - 1))
        } else {
            h
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_get() {
        let mut ht = HashTable::<usize>::new();
        ht.put(42, 100);
        ht.put(43, 200);
        ht.put(44, 300);
        ht.put(45, 400);
        assert_eq!(ht.get(42), Some(&100));
        assert_eq!(ht.get(43), Some(&200));
        assert_eq!(ht.get(44), Some(&300));
        assert_eq!(ht.get(45), Some(&400));
        assert_eq!(ht.get(46), None);
    }

    #[test]
    fn a_lot_of_items() {
        let mut ht = HashTable::<usize>::new();
        for i in 0..1000 {
            ht.put(i, 1000 + i);
        }
        for i in 0..1000 {
            assert_eq!(ht.get(i), Some(&(1000 + i)));
        }
        assert_eq!(ht.get(1000), None);

        for i in 100..200 {
            ht.remove(i);
        }
        for i in 0..1000 {
            if i >= 100 && i < 200 {
                assert_eq!(ht.get(i), None);
            } else {
                assert_eq!(ht.get(i), Some(&(1000 + i)));
            }
        }
    }

    #[test]
    fn remove() {
        let mut ht = HashTable::<usize>::new();
        ht.put(42, 100);
        ht.put(43, 200);
        assert_eq!(ht.get(42), Some(&100));
        assert_eq!(ht.get(43), Some(&200));
        ht.remove(42);
        assert_eq!(ht.get(42), None);
        assert_eq!(ht.get(43), Some(&200));
    }

    #[test]
    fn split() {
        let mut ht = HashTable::<usize>::new();
        ht.put(42, 100);
        ht.put(43, 200);
        ht.put(44, 300);
        ht.put(45, 400);
        ht.split();
        assert_eq!(ht.buckets.len(), 2);
        assert_eq!(ht.buckets[0].len(), 2);
        assert_eq!(ht.buckets[1].len(), 2);
    }
}
