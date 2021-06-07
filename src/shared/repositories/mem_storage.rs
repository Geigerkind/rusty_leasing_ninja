use std::collections::HashMap;
use std::sync::RwLock;
use std::hash::Hash;

#[derive(Debug)]
pub struct MemStorage<A: Eq + Hash, B: Clone>(RwLock<HashMap<A, B>>);

impl<A: Eq + Hash, B: Clone> MemStorage<A, B> {
    pub fn new() -> Self {
        MemStorage(RwLock::new(HashMap::default()))
    }

    pub fn save(&self, key: A, value: B) {
        let mut repo = self.0.write().unwrap();
        repo.insert(key, value);
    }

    pub fn get(&self, key: &A) -> Option<B> {
        let repo = self.0.read().unwrap();
        repo.get(key).cloned()
    }
}