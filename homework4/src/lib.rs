
#[derive(Debug)]
pub struct Vect<T, const N: usize>
{
    _data: [Option<T>; N],
    _capacity: usize,
    _length: usize,
    _path: Box<Option<Vect<T, N>>>
}

impl<T, const N: usize> Vect<T, N> {

    pub fn new(array: [T; N]) -> Vect<T, N> {
        let len = array.len();
        Vect {
            _data: array.map(|e| Some(e)),
            _capacity: len,
            _length: len,
            _path: Box::new(None),
            
        }
    }

    pub fn with_capacity() -> Vect<T, N> {
        Vect {
            _data: [0; N].map(|_| None),
            _capacity: N,
            _length: 0,
            _path: Box::new(None),
        }
    }

    pub fn get(&self, index: usize) -> &T 
    {
        if index >= self._length {
            panic!("out of range")
        } else if index >= self._data.len() {
            if let Some(e) = self._path.as_ref() {
                e.get(index - self._data.len())
            } else {
                panic!("out of range")
            }
        } else {
            match &self._data[index] {
                Some(e) => {return e},
                None => panic!("out of range")
            }
        }
    }

    pub fn get_mut(&mut self, index: usize) -> &mut T {
        if index >= self._length {
            panic!("out of range")
        } else if index >= self._data.len() {
            if let Some(e) = self._path.as_mut() {
                e.get_mut(index - self._data.len())
            } else {
                panic!("out of range")
            }
        } else {
            match &mut self._data[index] {
                Some(e) => {return e},
                None => panic!("out of range")
            }
        }
    }

    pub fn capacity(&self) -> usize {
        self._capacity
    }

    pub fn length(&self) -> usize {
        self._length
    }

    pub fn push(&mut self, elment: T) {   
        
        self._length += 1;
        
        if let Some(e) = self._path.as_mut() {
            if self._length > self._capacity {
                self._capacity += N;
            }
            if let Some(Some(_)) = self._data.last() {
                e.push(elment);
            } else {
                for i in self._data.iter_mut() {
                    if let None = i {
                        *i = Some(elment);
                        break;
                    }
                }
            }
            
        } else if let None = self._path.as_mut() {

            if self._length > self._capacity {
                self._capacity += N;
                
                let mut path = Vect {
                    //  ?????? (Возможно строка снизу костыль и наверное есть вариант получше)
                    _data: [0; N].map(|_| None),
                    _capacity: N,
                    _length: 1,
                    _path: Box::new(None),
                };
                path._data[0] = Some(elment); 
    
                self._path = Box::new(Some(path));
            } else {
                for i in self._data.iter_mut() {
                    if let None = i {
                        *i = Some(elment);
                        break;
                    }
                }
            }
            
        }
        
    }

    pub fn pop(&mut self) -> T where T: Copy
    {   
        self._length -= 1;
        let last_element: T;
        if let Some(Some(e)) = self._data.last() {
            if let Some(el) = self._path.as_mut() {
                if let Some(Some(_)) = el._data.first() {
                    el.pop()
                } else {
                    last_element = *e;
                    match self._data.last_mut() {
                        Some(e) => *e = None,
                        None => {panic!("vect is nulll")},
                    }
                return  last_element;
                }
            } else {
                last_element = *e;
                match self._data.last_mut() {
                    Some(e) => *e = None,
                    None => {panic!("vect is nulll")},
                }
                return  last_element;
            }
        } else {
            for i in self._data.iter_mut().rev() {
                match i {
                    Some(e) => {
                        last_element = *e;
                        *i = None;
                        return last_element;
                    },
                    None => continue,
                }
            }
            panic!("vect is null")
        }
        
    }

    
    pub fn remove(&mut self, index: usize) -> T where T: Copy {
        let out = *self.get_mut(index);
        for i in index..(self._length - 1) {
            *self.get_mut(i) = *self.get(i + 1);
        }
        self.pop();
        out      
    }

    pub fn resize(&mut self, new_len: usize, value: T) where T: Copy {
        while self._length > new_len {
            self.pop();
        }
        while self._length < new_len {
            self.push(value);
        }
    }

}
