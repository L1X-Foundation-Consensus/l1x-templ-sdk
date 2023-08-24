mod impls;

use super::IndexMap;
use borsh::{BorshDeserialize, BorshSerialize};

const ERR_INDEX_OUT_OF_BOUNDS: &str = "Index out of bounds";

pub struct Vector<T>
where
    T: BorshSerialize + BorshDeserialize,
{
    pub(crate) len: u32,
    pub(crate) values: IndexMap<T>,
}

impl<T> BorshSerialize for Vector<T>
where
    T: BorshSerialize + BorshDeserialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), borsh::maybestd::io::Error> {
        BorshSerialize::serialize(&self.len, writer)?;
        BorshSerialize::serialize(&self.values, writer)?;
        Ok(())
    }
}

impl<T> BorshDeserialize for Vector<T>
where
    T: BorshSerialize + BorshDeserialize,
{
    fn deserialize(
        buf: &mut &[u8],
    ) -> Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            len: BorshDeserialize::deserialize(buf)?,
            values: BorshDeserialize::deserialize(buf)?,
        })
    }
}

impl<T> Vector<T>
where
    T: BorshSerialize + BorshDeserialize,
{
    pub fn new(prefix: Vec<u8>) -> Self {
        Self { len: 0, values: IndexMap::new(prefix) }
    }

    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn flush(&mut self) {
        self.values.flush();
    }

    pub fn set(&mut self, index: u32, value: T) {
        if index >= self.len() {
            crate::panic(ERR_INDEX_OUT_OF_BOUNDS);
        }

        self.values.set(index, Some(value));
    }

    pub fn push(&mut self, element: T) {
        let last_idx = self.len();
        self.len = self
            .len
            .checked_add(1)
            .unwrap_or_else(|| crate::panic(ERR_INDEX_OUT_OF_BOUNDS));
        self.set(last_idx, element)
    }

    pub fn get(&self, index: u32) -> Option<&T> {
        if index >= self.len() {
            return None;
        }
        self.values.get(index)
    }

    pub fn get_mut(&mut self, index: u32) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }
        self.values.get_mut(index)
    }
}

//====================================================== TESTS =================================================================

#[cfg(test)]
mod tests {
    use super::super::super::tests::*;
    use super::*;
    use borsh::{BorshDeserialize, BorshSerialize};

    #[derive(BorshSerialize, BorshDeserialize, PartialEq, Clone, Debug)]
    struct TestValue(i32);

    #[test]
    fn test_vector_new_and_len() {
        let vector: Vector<TestValue> = Vector::new(b"test".to_vec());
        assert_eq!(vector.len(), 0);
    }

    #[test]
    fn test_vector_push_and_get() {
        let mut vector: Vector<TestValue> = Vector::new(b"test".to_vec());
        vector.push(TestValue(10));
        assert_eq!(vector.len(), 1);
        assert_eq!(vector.get(0), Some(&TestValue(10)));
    }

    #[test]
    fn test_vector_set_and_get_mut() {
        let mut vector: Vector<TestValue> = Vector::new(b"test".to_vec());
        vector.push(TestValue(10));
        vector.set(0, TestValue(20));
        assert_eq!(vector.get(0), Some(&TestValue(20)));
        if let Some(value) = vector.get_mut(0) {
            *value = TestValue(30);
        }
        assert_eq!(vector.get(0), Some(&TestValue(30)));
    }

    #[test]
    fn test_vector_out_of_bounds() {
        let vector: Vector<TestValue> = Vector::new(b"test".to_vec());
        assert_eq!(vector.get(0), None);
    }

    #[test]
    fn test_vector_non_empty_prefix() {
        let vector: Vector<TestValue> =
            Vector::new(b"non_empty_prefix".to_vec());
        assert_eq!(vector.len(), 0);
    }

    #[test]
    fn test_vector_push_multiple() {
        let mut vector: Vector<TestValue> = Vector::new(b"test".to_vec());
        vector.push(TestValue(10));
        vector.push(TestValue(20));
        vector.push(TestValue(30));
        assert_eq!(vector.len(), 3);
        assert_eq!(vector.get(0), Some(&TestValue(10)));
        assert_eq!(vector.get(1), Some(&TestValue(20)));
        assert_eq!(vector.get(2), Some(&TestValue(30)));
    }

    #[test]
    #[ignore]
    fn test_push_persistence() {
        let mut vector: Vector<TestValue> = Vector::new(b"test".to_vec());

        vector.push(TestValue(10));

        // Construct the expected storage key
        let mut expected_key = b"test".to_vec();
        expected_key.extend_from_slice(&0u32.to_le_bytes());

        // Check that the value has been written in the underlying storage
        let written_value = TestValue::try_from_slice(
            &mut &*storage_read(&expected_key).unwrap(),
        )
        .unwrap();
    }

    #[test]
    #[ignore]
    fn test_set_persistence() {
        let mut vector: Vector<TestValue> = Vector::new(b"test".to_vec());

        vector.push(TestValue(10));
        vector.set(0, TestValue(20));

        // Construct the expected storage key
        let mut expected_key = b"test".to_vec();
        expected_key.extend_from_slice(&0u32.to_le_bytes());

        // Check that the value has been updated in the underlying storage
        let written_value = TestValue::try_from_slice(
            &mut &*storage_read(&expected_key).unwrap(),
        )
        .unwrap();
    }
}
