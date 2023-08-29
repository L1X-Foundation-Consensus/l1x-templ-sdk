//! An implementation of a set that stores its content directly on the persistent storage.
mod impls;

use borsh::{BorshDeserialize, BorshSerialize};
use std::borrow::Borrow;
use std::marker::PhantomData;

/// An implementation of a set that stores its content directly on the persistent storage.
#[derive(BorshSerialize, BorshDeserialize)]
pub struct LookupSet<T>
where
    T: BorshSerialize,
{
    prefix: Box<[u8]>,

    #[borsh_skip]
    hasher: PhantomData<fn() -> (T, Vec<u8>)>,
}

fn to_key<Q: ?Sized>(prefix: &[u8], key: &Q, buffer: &mut Vec<u8>) -> Vec<u8>
where
    Q: BorshSerialize,
{
    // Prefix the serialized bytes and return a copy of this buffer.
    buffer.extend(prefix);
    key.serialize(buffer).unwrap_or_else(|_| crate::abort());

    buffer.clone()
}

impl<T> LookupSet<T>
where
    T: BorshSerialize,
{
    /// Creates a new set. Uses `prefix` as a unique prefix for keys.
    pub fn new(prefix: Vec<u8>) -> Self {
        Self {
            prefix: prefix.into_boxed_slice(),
            hasher: Default::default(),
        }
    }

    /// Returns true if the set contains a value.
    pub fn contains<Q: ?Sized>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: BorshSerialize,
    {
        let lookup_key = to_key(&self.prefix, value, &mut Vec::new());
        crate::storage_read(lookup_key.as_ref()).is_some()
    }

    /// Adds a value to the set.
    ///
    /// Returns whether the value was newly inserted. That is:
    ///
    /// * If the set did not previously contain this value, true is returned.
    /// * If the set already contained this value, false is returned.
    pub fn insert(&mut self, value: T) -> bool {
        let lookup_key = to_key(&self.prefix, &value, &mut Vec::new());
        !crate::storage_write(lookup_key.as_ref(), &[])
    }

    /// Removes a value from the set. Returns whether the value was present in the set.
    pub fn remove<Q: ?Sized>(&mut self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: BorshSerialize,
    {
        let lookup_key = to_key(&self.prefix, value, &mut Vec::new());
        crate::storage_remove(lookup_key.as_ref())
    }
}

//======================================================= TESTS =======================================================================

#[cfg(test)]
mod tests {
    use super::super::super::tests::*;
    use super::*;

    use borsh::{BorshDeserialize, BorshSerialize};

    #[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
    struct TestValue(i32);

    #[test]
    fn test_new() {
        let set: LookupSet<TestValue> = LookupSet::new(b"test".to_vec());
        assert_eq!(&*set.prefix, b"test");
    }

    #[test]
    #[ignore]
    fn test_insert() {
        let mut set: LookupSet<TestValue> = LookupSet::new(b"test".to_vec());

        // Insert value
        assert!(set.insert(TestValue(10)));

        // Inserting the same value again should return false
        assert!(!set.insert(TestValue(10)));
    }

    #[test]
    fn test_contains() {
        let mut set: LookupSet<TestValue> = LookupSet::new(b"test".to_vec());

        // Insert value
        set.insert(TestValue(10));

        // Check for inserted value
        assert!(set.contains(&TestValue(10)));

        // Check for non-inserted value
        assert!(!set.contains(&TestValue(20)));
    }

    #[test]
    #[ignore]
    fn test_insert_duplicate_values() {
        let mut set: LookupSet<TestValue> = LookupSet::new(b"test".to_vec());

        // Insert a value
        assert!(set.insert(TestValue(1)));

        // Try to insert the same value again. This time it should return false because it is a duplicate.
        assert!(!set.insert(TestValue(1)));
    }

    #[test]
    fn test_contains_non_existent_value() {
        let set: LookupSet<TestValue> = LookupSet::new(b"test".to_vec());

        // Check for a value that hasn't been inserted
        assert!(!set.contains(&TestValue(15)));
    }

    #[test]
    #[ignore]
    fn test_insert_persistence() {
        let mut set: LookupSet<TestValue> = LookupSet::new(b"test".to_vec());

        // Insert value
        assert!(set.insert(TestValue(10)));

        // Check storage for value
        let lookup_key = to_key(&set.prefix, &TestValue(10), &mut Vec::new());
        let stored_value = crate::storage_read(lookup_key.as_ref());

        assert!(
            stored_value.is_some(),
            "Expected the value to be set in storage"
        );
    }

    #[test]
    fn test_remove() {
        let mut lookup_set: LookupSet<u32> = LookupSet::new(vec![0, 1, 2]);

        // Insert values
        lookup_set.insert(10);
        lookup_set.insert(20);
        lookup_set.insert(30);

        // Check if values exist
        assert_eq!(lookup_set.contains(&10), true);
        assert_eq!(lookup_set.contains(&20), true);
        assert_eq!(lookup_set.contains(&30), true);

        // Remove values
        assert_eq!(lookup_set.remove(&10), true);
        assert_eq!(lookup_set.remove(&20), true);

        // Check if values exist after removing
        assert_eq!(lookup_set.contains(&10), false);
        assert_eq!(lookup_set.contains(&20), false);
        assert_eq!(lookup_set.contains(&30), true);

        // Remove a value not present in the set
        assert_eq!(lookup_set.remove(&40), false);
    }
}
