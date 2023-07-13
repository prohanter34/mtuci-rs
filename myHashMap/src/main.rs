use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hasher, Hash};

pub struct Pair<T, U> where T: Hash  {
    _key: T,
    _value: U,
    _path: Option<Box<Pair<T, U>>>
}

impl<T, U> Pair<T, U> where T: Hash {
    
    pub fn insert(&mut self, key: T, value: U) {
        if let Some(mut e) = self._path.as_mut() {
            e.as_mut().insert(key, value)
        } else {
            self._key = key;
            self._value = value;
        }
    }

    pub fn get(&self, key: T) -> Option<&U> {
        todo!()
    }

}

pub struct HashM<T, U> where T: Hash {
    _data: [Option<Pair<T, U>>; 10000]
}

impl<T, U> HashM<T, U> where T: Hash {

    pub fn new() -> HashM<T, U> {
        HashM { _data: [1; 10000].map(|_| None) }
    }

    pub fn insert(&mut self, key: T, value: U) {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        let mut hash = s.finish() as usize;
        hash = hash % 10000;
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
        hash = hash % 10000;
        match &self._data[hash] {
            Some(e) => {
                if let Some(path) = e._path.as_ref() {                 // доделать
                    return path.get(key);
                } else {
                    return Some(&e._value);
                }
            },
            None => None,
        }
    }

}

fn main() {
    let mut s = DefaultHasher::new();
    let str = [23; 8];
    str.hash(&mut s);
    print!("{:?}", s.finish().to_string().len());
}
