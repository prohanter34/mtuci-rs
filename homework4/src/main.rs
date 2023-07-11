use std::{fmt::Debug, vec, clone};


#[derive(Debug)]
struct Vect<T, const N: usize>
{
    _data: [Option<T>; N],
    _capacity: usize,
    _length: usize,
    _path: Box<Option<Vect<T, N>>>
}

impl<T, const N: usize> Vect<T, N> {

    fn new(array: [T; N]) -> Vect<T, N> {
        let len = array.len();
        Vect {
            _data: array.map(|e| Some(e)),
            _capacity: len,
            _length: len,
            _path: Box::new(None),
            
        }
    }

    fn with_capacity() -> Vect<T, N> {
        Vect {
            _data: [0; N].map(|_| None),
            _capacity: N,
            _length: 0,
            _path: Box::new(None),
        }
    }

    fn get(&self, index: usize) -> &T 
    // where T: Copy
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

    fn get_mut(&mut self, index: usize) -> &mut T {
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

    fn capacity(&self) -> usize {
        self._capacity
    }

    fn length(&self) -> usize {
        self._length
    }
    // не хватает условия !!!!!!  СДЕЛАТЬ!
    fn push(&mut self, elment: T) {   
        
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

    fn pop(&mut self) -> T where T: Copy
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
            }                                                                   // дописать!!!
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

    
    fn remove(&mut self, index: usize) -> T {
        todo!()
    }

    fn resize(&mut self, new_len: usize, value: T) where T: Copy {
        while self._length > new_len {
            self.pop();
        }
        while self._length < new_len {
            self.push(value);
        }
    }

}


fn main() {
    // пример: new, push, with_capacity
    // адекватного нет(((
    let mut a = Vect::new([String::from("de")]); // new должен знать кол-во элементов массива во время компиляции (плохо) 
    a.push("elment".to_string());
    println!("{:?}", a);
    let mut a: Vect<usize, 2> = Vect::with_capacity(); // Емкость передаётся не в аргументы, а в параметр (плохо) (проблема как и с new)
    // пример работы пуша и изменения ёмкости (увеличивается на число равное начальной ёмкости (в оригинале удваивается)) 
    a.push(1); 
    a.push(2); 
    a.push(3); 
    a.push(4); 
    a.push(5); 
    println!("{}", a.capacity()); 
    println!("{}", a.length()); 
    println!("{}", a.get(3));
    println!("{:?}", a);
    *a.get_mut(0) = 100;
    println!("{}", a.get(0));
    println!("{}", a.pop());
    println!("{}", a.pop());
    a.push(7);
    println!("{:?}", a);
    a.resize(2, 0);
    println!("{:?}", a);
}
