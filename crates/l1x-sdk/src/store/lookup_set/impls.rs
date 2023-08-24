use super::LookupSet;
use borsh::BorshSerialize;

impl<T> Extend<T> for LookupSet<T>
where
    T: BorshSerialize,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        iter.into_iter().for_each(move |elem| {
            self.insert(elem);
        });
    }
}
