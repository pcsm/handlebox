use std::collections::HashMap;

pub type Handle = usize;

/// A HashMap that keeps track of unused keys
#[derive(Default)]
pub struct HandleBox<V> {
    pub hash_map: HashMap<Handle, V>,
    discarded_handles: Vec<Handle>, // Handles that have been added but later removed
}

impl<V> HandleBox<V> {
    pub fn new() -> HandleBox<V> {
        HandleBox {
            hash_map: HashMap::new(),
            discarded_handles: vec![],
        }
    }

    fn new_handle(&mut self) -> Handle {
        self.discarded_handles.pop().unwrap_or(self.hash_map.values().len())
    }

    pub fn add(&mut self, value: V) -> Handle {
        let h = self.new_handle();
        self.hash_map.insert(h, value);
        h
    }

    pub fn remove(&mut self, handle: &Handle) {
        let result = self.hash_map.remove(handle);
        if result.is_some() {
            self.discarded_handles.push(*handle)
        }
    }

    pub fn get(&self, handle: &Handle) -> Option<&V> {
        self.hash_map.get(handle)
    }

    pub fn get_mut(&mut self, handle: &Handle) -> Option<&mut V> {
        self.hash_map.get_mut(handle)
    }

    pub fn hash_map(&self) -> &HashMap<Handle, V> {
        &self.hash_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let c = HandleBox::<i32>::new();
        assert!(c.hash_map().is_empty());
    }

    #[test]
    fn test_add_remove() {
        let mut c = HandleBox::new();
        let h1 = c.add(888);
        assert!(!c.hash_map().is_empty());
        assert_eq!(c.get(&h1).unwrap(), &888);

        let h2 = c.add(999);
        assert_eq!(c.hash_map().values().len(), 2);
        assert_eq!(c.get(&h2).unwrap(), &999);

        c.remove(&h2);
        assert_eq!(c.hash_map().values().len(), 1);
        assert!(c.get(&h2).is_none());

        c.remove(&h1);
        assert!(c.hash_map().is_empty());
        assert!(c.get(&h1).is_none());
    }
}