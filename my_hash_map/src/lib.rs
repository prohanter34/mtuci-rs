use std::collections::hash_map::DefaultHasher;
use std::hash::{Hasher, Hash};

#[derive(Debug)]
pub struct Pair<T, U> where T: Hash + Eq  {
    _key: T,
    _value: U,
    _path: Option<Box<Pair<T, U>>>
}


impl<T, U> Pair<T, U> where T: Hash + Eq {
    
    pub fn insert(&mut self, key: T, value: U) {
        if let Some(e) = self._path.as_mut() {
            e.as_mut().insert(key, value)
        } else {
            if self._key == key {
                self._value = value;
            } else {
                self._path = Some(Box::new(Pair { _key: key, _value: value, _path: None }));
            }
        }
    }

    pub fn get(&self, key: T) -> Option<&U> {
        if let Some(e) = &self._path {
            e.get(key)
        } else {
            Some(&self._value)
        }
    }

    pub fn remove(&mut self, key: T) -> Option<U> {
        todo!();
    }

}

#[derive(Debug)]
pub struct HashM<T, U> where T: Hash + Eq {
    _data: [Option<Pair<T, U>>; 1000]
}

impl<T, U> HashM<T, U> where T: Hash + Eq {

    pub fn new() -> HashM<T, U> {
        HashM { _data: [1; 1000].map(|_| None) }
    }

    pub fn insert(&mut self, key: T, value: U) {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        let mut hash = s.finish() as usize;
        hash = hash % 1000;
        if let Some(e) = self._data[hash].as_mut() {
            e.insert(key, value);
        } else {
            self._data[hash] = Some(Pair {_key: key, _value: value, _path: None });  
        }
    }

    pub fn get(&self, key: T) -> Option<&U> {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        let mut hash = s.finish() as usize;
        hash = hash % 1000;
        match &self._data[hash] {
            Some(e) => {
                if let Some(path) = e._path.as_ref() {
                    return path.get(key);
                } else {
                    return Some(&e._value);
                }
            },
            None => None,
        }
    }

    pub fn remove(&mut self, key: T) -> Option<U> where U: Copy {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        let mut hash = s.finish() as usize;
        hash = hash % 1000;
        if let Some(e) = self._data[hash].as_mut() {
            if let Some(path) = e._path.as_mut() {
                path.remove(key)
            } else {
                let out = e._value;
                self._data[hash] = None;
                return Some(out);
            }
        } else {
            None
        }
    }

}