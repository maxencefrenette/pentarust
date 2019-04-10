use crate::game::Board;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

pub struct TranspositionTable {
    vec: Vec<Entry>,
}

impl TranspositionTable {
    pub fn new(size: usize) -> TranspositionTable {
        let mut vec = Vec::with_capacity(size);

        for _ in 0..size {
            vec.push(Entry::default());
        }

        TranspositionTable { vec }
    }

    pub fn put(&mut self, key: Board, value: i32, depth: u32) {
        let i = self.index(key);
        self.vec[i] = Entry { key, value, depth };
    }

    pub fn get(&self, key: Board, min_depth: u32) -> Option<i32> {
        let i = self.index(key);
        let entry = &self.vec[i];

        // if entry.key == key && entry.depth >= min_depth {
        //     Some(entry.value)
        // } else {
            None
        // }
    }

    fn index(&self, key: Board) -> usize {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        s.finish() as usize % self.vec.len()
    }
}

#[derive(Debug, Default)]
struct Entry {
    key: Board,
    value: i32,
    depth: u32,
}
