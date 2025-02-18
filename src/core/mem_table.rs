use std::{collections::BTreeSet, iter::IntoIterator};

use crate::core::entry::flexible_entry::FlexibleEntry;

use super::field::FlexibleField;

pub struct MemoryTable {
    entries: BTreeSet<FlexibleEntry>,
    current_size: usize,
    max_table_size: usize,
}

impl MemoryTable {
    pub fn new(max_table_size: usize) -> Self {
        let entries = BTreeSet::new();

        MemoryTable {
            entries,
            current_size: 0,
            max_table_size,
        }
    }

    pub fn current_size(&self) -> usize {
        self.current_size
    }

    pub fn max_table_size(&self) -> usize {
        self.max_table_size
    }

    pub fn append(&mut self, row: FlexibleEntry) {
        self.entries.insert(row);
        self.current_size += 1;
    }

    pub fn get_value(&self, key: &FlexibleField) -> Option<FlexibleField> {
        self.entries
            .iter()
            .find(|entry| entry.get_key() == key)
            .map(|entry| entry.get_value().clone())
    }

    pub fn iter(&self) -> impl Iterator<Item = &FlexibleEntry> {
        self.entries.iter()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.current_size = 0;
    }
}

pub struct MemoryTableIterator<'a> {
    it: Box<dyn Iterator<Item = &'a FlexibleEntry> + 'a>,
}

impl<'a> Iterator for MemoryTableIterator<'a> {
    type Item = &'a FlexibleEntry;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next()
    }
}

impl<'a> IntoIterator for &'a MemoryTable {
    type Item = &'a FlexibleEntry;
    type IntoIter = MemoryTableIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MemoryTableIterator {
            it: Box::new(self.entries.iter()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::entry::flexible_entry::FlexibleEntry;
    use crate::core::field::*;
    use crate::core::mem_table;
    use std::iter::zip;

    #[test]
    fn check_sizes() {
        let mem_table = mem_table::MemoryTable::new(3);
        assert_eq!(mem_table.current_size(), 0);
        assert_eq!(mem_table.max_table_size(), 3);
    }

    #[test]
    fn check_append() {
        let mut mem_table = mem_table::MemoryTable::new(3);

        let entry1 = FlexibleEntry::new(
            FlexibleField::new(vec![1, 2, 3]),
            FlexibleField::new(vec![10, 20, 30]),
        );
        mem_table.append(entry1.clone());

        let entry2 = FlexibleEntry::new(
            FlexibleField::new(vec![2, 3, 4]),
            FlexibleField::new(vec![20, 30, 40]),
        );
        mem_table.append(entry2.clone());

        let entry3 = FlexibleEntry::new(
            FlexibleField::new(vec![3, 4, 5]),
            FlexibleField::new(vec![30, 40, 50]),
        );
        mem_table.append(entry3.clone());

        let mut it = mem_table.iter();

        let r = it.next();
        assert_eq!(&entry1, r.unwrap());

        let r = it.next();
        assert_eq!(&entry2, r.unwrap());

        let r = it.next();
        assert_eq!(&entry3, r.unwrap());

        assert!(it.next().is_none());

        let expected = [&entry1, &entry2, &entry3];
        for (actual, expected) in zip(&mem_table, expected) {
            assert_eq!(actual, expected);
        }
    }
}
