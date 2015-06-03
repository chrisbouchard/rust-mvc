use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::usize;

#[derive(Debug)]
struct QueueEntry<V> {
    value: V,
    count: usize
}

impl<V> QueueEntry<V> {
    fn new(value: V, count: usize) -> QueueEntry<V> {
        QueueEntry {
            value: value,
            count: count
        }
    }
}


#[derive(Debug)]
pub struct QueueMap<K, V> where K: Hash + Eq, V: Copy {
    queue: VecDeque<QueueEntry<V>>,
    indeces: HashMap<K, usize>,
    dangling: usize,
    shift: usize
}

impl<K, V> QueueMap<K, V> where K: Hash + Eq, V: Copy {
    pub fn new() -> QueueMap<K, V> {
        QueueMap {
            queue: VecDeque::new(),
            indeces: HashMap::new(),
            dangling: 0,
            shift: 0
        }
    }

    pub fn add(&mut self, key: K) {
        self.indeces.insert(key, self.queue.len() + self.shift);
        self.dangling += 1;
    }

    pub fn push(&mut self, value: V) {
        let include_prev =
            if self.queue.is_empty() { 0 } else { 1 };

        self.queue.push_back(QueueEntry::new(value, self.dangling + include_prev));
        self.dangling = 0;

        if self.shift + self.queue.len() == usize::MAX {
            self.compact();
        }
    }

    pub fn pop(&mut self, key: &K) -> Option<V> {
        let mut needs_shift = false;
        let mut result = None;

        if let Some(index) = self.indeces.get_mut(key) {
            if let Some(entry) = self.queue.get_mut(*index - self.shift) {
                result = Some(entry.value);

                *index += 1;
                entry.count -= 1;

                if entry.count == 0 {
                    needs_shift = true;
                }
            }

            if let Some(entry) = self.queue.get_mut(*index - self.shift) {
                entry.count += 1;
            }
            else {
                self.dangling += 1;
            }
        }

        while needs_shift {
            self.queue.pop_front();
            self.shift += 1;

            needs_shift =
                match self.queue.front_mut() {
                    None => false,
                    Some(front) => {
                        front.count -= 1;
                        front.count == 0
                    }
                };
        }

        if self.shift + self.queue.len() == usize::MAX {
            self.compact();
        }

        result
    }

    pub fn is_empty(&self, key: &K) -> bool {
        self.indeces.get(key).map_or(true, |index| {
            (*index - self.shift) >= self.queue.len()
        })
    }

    pub fn compact(&mut self) {
        for (_, index) in self.indeces.iter_mut() {
            *index -= self.shift;
        }

        self.shift = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::QueueMap;

    #[test]
    fn test_new() {
        let _: QueueMap<usize, usize> = QueueMap::new();
    }

    #[test]
    fn test_simple() {
        let mut queue_map: QueueMap<usize, usize> = QueueMap::new();

        queue_map.add(1);
        queue_map.push(2);

        assert_eq!(Some(2), queue_map.pop(&1));
        assert_eq!(None, queue_map.pop(&1));
    }

    #[test]
    fn test_two_queues() {
        let mut queue_map: QueueMap<usize, usize> = QueueMap::new();

        queue_map.add(1);
        queue_map.add(2);

        queue_map.push(1);
        queue_map.push(2);

        assert_eq!(Some(1), queue_map.pop(&1));

        queue_map.add(3);

        queue_map.push(3);

        assert_eq!(Some(1), queue_map.pop(&2));

        assert_eq!(Some(3), queue_map.pop(&3));

        assert_eq!(Some(2), queue_map.pop(&1));
        assert_eq!(Some(2), queue_map.pop(&2));

        assert_eq!(Some(3), queue_map.pop(&1));
        assert_eq!(Some(3), queue_map.pop(&2));

        assert_eq!(None, queue_map.pop(&1));
        assert_eq!(None, queue_map.pop(&2));
        assert_eq!(None, queue_map.pop(&3));
    }
}

