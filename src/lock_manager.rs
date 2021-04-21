// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::collections::HashMap;

#[derive(PartialEq, Eq)]
enum Lock {
    Shared(usize),
    Exclusive,
}

pub struct LockManager {
    locks: HashMap<usize, Lock>,
}

impl LockManager {
    pub fn new() -> Self {
        Self {
            locks: HashMap::new(),
        }
    }

    /// Acquire a shared (i.e. read-only) lock.
    /// Fails iff the object is currently locked exclusively.
    pub fn lock_shared(&mut self, id: usize) -> bool {
        match self.locks.get(&id) {
            Some(Lock::Exclusive) => false,
            Some(Lock::Shared(n)) => {
                let new_n = n + 1;
                self.locks.insert(id, Lock::Shared(new_n));
                true
            }
            None => {
                self.locks.insert(id, Lock::Shared(1));
                true
            }
        }
    }

    /// Acquire an exclusive (i.e. read/write) lock.
    /// Fails iff the object is currently locked (shared or exclusive).
    pub fn lock_exclusive(&mut self, id: usize) -> bool {
        if self.locks.get(&id).is_some() {
            return false;
        }
        self.locks.insert(id, Lock::Exclusive);
        return true;
    }

    //pub fn lock_upgrade(&mut self, id: usize) {}

    pub fn unlock(&mut self, id: usize) {
        if let Some(Lock::Shared(n)) = self.locks.get(&id) {
            if *n > 1 {
                let new_n = n - 1;
                self.locks.insert(id, Lock::Shared(new_n));
                return;
            }
        }
        self.locks.remove(&id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut lm = LockManager::new();

        for _ in 0..5 {
            assert_eq!(lm.lock_shared(0), true);
            assert_eq!(lm.lock_exclusive(0), false);
        }
        assert_eq!(lm.lock_exclusive(1), true);

        for _ in 0..5 {
            assert_eq!(lm.lock_exclusive(0), false);
            lm.unlock(0);
        }
        assert_eq!(lm.lock_exclusive(0), true);

        lm.unlock(1);
        for _ in 0..5 {
            assert_eq!(lm.lock_shared(1), true);
        }
    }
}
