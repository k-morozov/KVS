use crate::core::field::FieldSize;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct Entry<K, V>(K, V);

impl<K, V> Entry<K, V>
where
    K: FieldSize,
    V: FieldSize,
{
    pub fn new(key: K, value: V) -> Self {
        Entry { 0: key, 1: value }
    }

    pub fn get_key(&self) -> &K {
        &self.0
    }

    pub fn get_mut_key(&mut self) -> &mut K {
        &mut self.0
    }

    pub fn get_value(&self) -> &V {
        &self.1
    }

    pub fn get_mut_value(&mut self) -> &mut V {
        &mut self.1
    }

    pub fn size(&self) -> usize {
        self.0.size() + self.1.size()
    }
}
