use std::collections::BTreeMap;

/// The keys used in a `HandleBox`. A typedef for `u32`.
pub type Handle = u32;

/// A map-like collection that reuses unused keys. Hard-coded to use `u32` `Handle`s right now.
#[derive(Default)]
pub struct HandleBox<V> {
    /// If you need to directly access the internal `BTreeMap`, you can use this field.
    ///
    /// Do not remove items from the internal map directly, because then the `Handle`s
    /// can't be reused. Use the `HandleBox.remove()` method instead.
    pub map: BTreeMap<Handle, V>,
    discarded_handles: Vec<Handle>, // Handles that have been added but later removed
}

impl<V> HandleBox<V> {
    /// Create an empty `HandleBox`
    pub fn new() -> HandleBox<V> {
        HandleBox {
            map: BTreeMap::new(),
            discarded_handles: vec![],
        }
    }

    /// Add a new value to the `HandleBox`. Note that an associated `Handle` for
    /// this value is automatically created and returned. Duplicate insertions
    /// are possible if you insert the same value twice.
    pub fn add(&mut self, value: V) -> Handle {
        let h = self.new_handle();
        self.map.insert(h, value);
        h
    }

    /// Remove the value associated with this `Handle`. If the value was removed,
    /// `Some<V>` is returned. If not, `None` is returned.
    pub fn remove(&mut self, handle: &Handle) -> Option<V> {
        let result = self.map.remove(handle);
        if result.is_some() {
            self.discarded_handles.push(*handle)
        }
        result
    }

    /// Get a shared reference to the value with this `Handle`.
    pub fn get(&self, handle: &Handle) -> Option<&V> {
        self.map.get(handle)
    }

    /// Get a mutable reference to the value with this `Handle`.
    pub fn get_mut(&mut self, handle: &Handle) -> Option<&mut V> {
        self.map.get_mut(handle)
    }
    
    fn new_handle(&mut self) -> Handle {
        self.discarded_handles.pop().unwrap_or(self.map.values().len() as Handle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let c = HandleBox::<i32>::new();
        assert!(c.map.is_empty());
    }

    #[test]
    fn test_add_remove() {
        let mut c = HandleBox::new();
        let h1 = c.add(888);
        assert!(!c.map.is_empty());
        assert_eq!(c.get(&h1).unwrap(), &888);

        let h2 = c.add(999);
        assert_eq!(c.map.values().len(), 2);
        assert_eq!(c.get(&h2).unwrap(), &999);

        c.remove(&h2);
        assert_eq!(c.map.values().len(), 1);
        assert!(c.get(&h2).is_none());

        c.remove(&h1);
        assert!(c.map.is_empty());
        assert!(c.get(&h1).is_none());
    }

    #[test]
    fn test_add_remove_add() {
        let mut c = HandleBox::new();
        c.add(888);

        let h2 = c.add(999);
        c.remove(&h2);
        
        let h3 = c.add(555);
        assert_eq!(h2, h3); // Because the handle gets re-used
    }
}