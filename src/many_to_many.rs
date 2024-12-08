#![allow(unused)]

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Clone, Debug)]
pub struct ManyToMany<TK, TV>(HashMap<TK, HashSet<TV>>);

impl<TK, TV> ManyToMany<TK, TV> {
    pub fn new() -> Self {
        Self(Default::default())
    }
}

impl<TK: Eq + Hash, TV: Eq + Hash> ManyToMany<TK, TV> {
    pub fn insert(&mut self, key: TK, value: TV) -> bool {
        // result
        // .entry(from_v)
        // .and_modify(|set| {
        //     set.insert(to_v);
        // })
        // .or_insert({
        //     let mut set = HashSet::new();
        //     set.insert(to_v);
        //     set
        // });
        if let Some(set) = self.0.get_mut(&key) {
            set.insert(value)
        } else {
            let mut set = HashSet::new();
            let res = set.insert(value);
            self.0.insert(key, set);
            res
        }
    }
    pub fn outer(&self) -> &HashMap<TK, HashSet<TV>> {
        &self.0
    }
    pub fn outer_mut(&mut self) -> &mut HashMap<TK, HashSet<TV>> {
        &mut self.0
    }
    pub fn remove_all(&mut self, key: &TK) -> Option<HashSet<TV>> {
        self.0.remove(key)
    }
    pub fn remove(&mut self, key: &TK, value: &TV) -> bool {
        self.0.iter();
        if let Some(set) = self.0.get_mut(&key) {
            set.remove(value)
        } else {
            false
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&TK, &TV)> {
        self.0
            .iter()
            .flat_map(|(key, set)| set.iter().map(move |value| (key, value)))
    }
}

// impl<TK, TV> IntoIterator for ManyToMany<TK, TV> {
//     type Item;

//     type IntoIter;

//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }

impl<TK: Eq + Hash + Clone, TV: Eq + Hash> ManyToMany<TK, TV> {
    pub fn inner(&self, key: &TK) -> Option<&HashSet<TV>> {
        self.0.get(key)
    }
    pub fn inner_mut(&mut self, key: &TK) -> &mut HashSet<TV> {
        if !self.0.contains_key(key) {
            self.0.insert(key.clone(), HashSet::new());
        }

        self.0.get_mut(key).unwrap()
    }

    pub fn into_iter(self) -> impl Iterator<Item = (TK, TV)> {
        self.0
            .into_iter()
            .flat_map(|(key, set)| set.into_iter().map(move |value| (key.clone(), value)))
    }
    pub fn into_inverted(self) -> ManyToMany<TV, TK> {
        let mut result = ManyToMany::new();
        for (key, value) in self.into_iter() {
            result.insert(value, key);
        }
        result
    }
}

impl<TK: Eq + Hash + Clone, TV: Eq + Hash + Clone> ManyToMany<TK, TV> {
    pub fn inverted(&self) -> ManyToMany<TV, TK> {
        let mut result = ManyToMany::new();
        for (key, value) in self.iter() {
            result.insert(value.clone(), key.clone());
        }
        result
    }
}
