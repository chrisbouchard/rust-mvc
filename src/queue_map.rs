use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::usize;

struct QueueEntry<V> {
    value: V,
    count: usize
}

pub struct QueueMap<K, V> {
    queue: VecDeque<QueueEntry<V>>,
    indeces: HashMap<K, usize>,
    dangling: usize,
    shift: usize
}

impl<V> QueueEntry<V> {
    fn new(value: V, count: usize) -> QueueEntry<V> {
        QueueEntry { value: value, count: count }
    }
}

impl<K, V> QueueMap<K, V> where K: Hash + Eq, V: Copy {
    pub fn new() -> QueueMap<K, V> {
        QueueMap { queue: VecDeque::new(), indeces: HashMap::new(), dangling: 0, shift: 0 }
    }

    pub fn add(&mut self, key: K) {
        self.indeces.insert(key, self.queue.len() + self.shift);
        self.dangling += 1;
    }

    pub fn push(&mut self, value: V) {
        self.queue.push_back(QueueEntry::new(value, self.dangling));
        self.dangling = 0;

        if self.shift + self.queue.len() >= usize::MAX {
            self.compact();
        }
    }

    pub fn pop(&mut self, key: &K) -> Option<V> {
        let mut needs_shift = false;

        let result =
            match self.indeces.get_mut(key) {
                None => None,
                Some(index) =>
                    match self.queue.get_mut(*index - self.shift) {
                        None => None,
                        Some(entry) => {
                            let value = (*entry).value;

                            *index += 1;
                            entry.count -= 1;

                            if entry.count == 0 {
                                needs_shift = true;
                            }

                            Some(value)
                        }
                    }
            };

        if needs_shift {
            self.queue.pop_front();
            self.shift += 1;
        }

        if self.shift + self.queue.len() >= usize::MAX {
            self.compact();
        }

        result
    }

    pub fn is_empty(&self, key: &K) -> bool {
        match self.indeces.get(key) {
            None => true,
            Some(index) => (*index - self.shift) >= self.queue.len()
        }
    }

    pub fn compact(&mut self) {
        for (_, index) in self.indeces.iter_mut() {
            *index -= self.shift;
        }

        self.shift = 0;
    }
}

