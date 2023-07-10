use std::{fmt::Debug, vec};


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

    fn get_value(&self, index: usize) -> &T 
    // where T: Copy
    {
        if index >= self._length {
            panic!("out of range")
        } else if index >= self._data.len() {
            if let Some(e) = self._path.as_ref() {
                e.get_value(index - self._data.len())
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

    fn get_mut_value(&mut self, index: usize) -> &mut T {
        if index >= self._length {
            panic!("out of range")
        } else if index >= self._data.len() {
            if let Some(e) = self._path.as_mut() {
                e.get_mut_value(index - self._data.len())
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
            e.push(elment);
           
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

    fn pop(&mut self) -> &T {
        
        if let Some(Some(e)) = self._data.last() {
            if let Some(el) = self._path.as_mut() {
                el.pop()
            } else {
                e
            }                                                                   // дописать!!!
        } else {
            for i in self._data.iter().rev() {
                return match i {
                    Some(e) => {e},
                    None => continue,
                }
            }
            panic!("vect is null")
        }
    }

    fn remove(&mut self, index: usize) -> T {
        todo!()
    }

    fn get(&self) {
        todo!()
    }

    fn resize(&self) {
        todo!()
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
    println!("{}", a.get_value(0));
    println!("{}", a.get_value(3));
    println!("{}", a.get_value(4));
    println!("{:?}", a);
    *a.get_mut_value(0) = 100;
    println!("{}", a.get_value(0));
}
