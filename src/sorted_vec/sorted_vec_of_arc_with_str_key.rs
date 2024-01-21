use std::sync::Arc;

use crate::sorted_vec::{EntityWithStrKey, GetOrCreateEntry, InsertEntity};

use super::InsertIfNotExists;

pub struct SortedVecOfArcWithStrKey<TValue: EntityWithStrKey> {
    items: Vec<Arc<TValue>>,
}

impl<TValue: EntityWithStrKey> SortedVecOfArcWithStrKey<TValue> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
        }
    }

    // Returns the index of the inserted item and old item if it was replaced
    pub fn insert_or_replace(&mut self, item: Arc<TValue>) -> (usize, Option<Arc<TValue>>) {
        let insert_index = self
            .items
            .binary_search_by(|itm| itm.get_key().cmp(item.get_key()));

        match insert_index {
            Ok(index) => {
                let got = std::mem::replace(&mut self.items[index], item);
                (index, Some(got))
            }
            Err(index) => {
                self.items.insert(index, item);
                (index, None)
            }
        }
    }

    pub fn insert_or_if_not_exists<'s>(
        &'s mut self,
        key: &str,
    ) -> InsertIfNotExists<'s, Arc<TValue>> {
        let insert_index = self.items.binary_search_by(|itm| itm.get_key().cmp(key));

        match insert_index {
            Ok(index) => {
                return InsertIfNotExists::Exists(index);
            }
            Err(index) => {
                return InsertIfNotExists::Insert(InsertEntity::new(index, &mut self.items));
            }
        }
    }

    pub fn contains(&self, key: &str) -> bool {
        let result = self.items.binary_search_by(|itm| itm.get_key().cmp(key));
        result.is_ok()
    }

    pub fn get(&self, key: &str) -> Option<&Arc<TValue>> {
        let result = self.items.binary_search_by(|itm| itm.get_key().cmp(key));

        match result {
            Ok(index) => self.items.get(index),
            Err(_) => None,
        }
    }

    pub fn get_or_create<'s>(&'s mut self, key: &str) -> GetOrCreateEntry<'s, Arc<TValue>> {
        let index = self.items.binary_search_by(|itm| itm.get_key().cmp(key));

        match index {
            Ok(index) => GetOrCreateEntry::Get(&self.items[index]),
            Err(index) => GetOrCreateEntry::Create(InsertEntity::new(index, &mut self.items)),
        }
    }

    pub fn get_by_index(&self, index: usize) -> Option<&Arc<TValue>> {
        self.items.get(index)
    }

    pub fn get_from_key_to_up(&self, key: &str) -> &[Arc<TValue>] {
        let result = self.items.binary_search_by(|itm| itm.get_key().cmp(key));

        match result {
            Ok(index) => &self.items[index..],
            Err(index) => &self.items[index..],
        }
    }

    pub fn get_from_bottom_to_key(&self, key: &str) -> &[Arc<TValue>] {
        let result = self.items.binary_search_by(|itm| itm.get_key().cmp(key));

        match result {
            Ok(index) => &self.items[..=index],
            Err(index) => &self.items[..=index],
        }
    }

    pub fn remove(&mut self, key: &str) -> Option<Arc<TValue>> {
        let result = self.items.binary_search_by(|itm| itm.get_key().cmp(key));

        match result {
            Ok(index) => Some(self.items.remove(index)),
            Err(_) => None,
        }
    }

    pub fn clear(&mut self, max_capacity: Option<usize>) {
        self.items.clear();

        if let Some(max_capacity) = max_capacity {
            self.items.reserve(max_capacity);
        }
    }

    pub fn first(&self) -> Option<&Arc<TValue>> {
        self.items.first()
    }

    pub fn last(&self) -> Option<&Arc<TValue>> {
        self.items.last()
    }

    pub fn iter(&self) -> std::slice::Iter<Arc<TValue>> {
        self.items.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<Arc<TValue>> {
        self.items.iter_mut()
    }

    pub fn into_vec(self) -> Vec<Arc<TValue>> {
        self.items
    }

    pub fn as_slice(&self) -> &[Arc<TValue>] {
        self.items.as_slice()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}
