use std::fmt::Debug;

#[derive(Debug)]
struct Vect<T, const N: usize>
// where T: Copy + Clone + Debug + IntoIterator + PartialEq + PartialOrd + Eq + Ord + Hash,
{
    _data: [Option<T>; N],
    _capacity: usize,
    _length: usize
}

impl<T, const N: usize> Vect<T, N> {

    fn new(array: [T; N]) -> Vect<T, N> {
        let len = array.len();
        Vect {
            _data: array.map(|e| Some(e)),
            _capacity: len,
            _length: len
        }
    }

    fn get_value(&self, index: usize) -> &T 
    // where T: Copy
    {
        match &self._data[index] {
            Some(e) => {return e},
            None => panic!("out of range")
        }
    }

    fn get_mut_value(&mut self, index: usize) -> &mut T {
        match &mut self._data[index] {
            Some(e) => {return e;},
            None => panic!("out of range")
        }
    }

    fn capacity(&self) -> usize {
        self._capacity
    }
    //Вот тут пока всё плохо
    fn push(&mut self, elment: T) 
    where T: Copy
    {
        self._length += 1;
        if self._length > self._capacity {
            self._capacity *= 2;
            let new_array = self._data.iter();
            // let mut new_array = [Option::<T>::None; self._capacity]; 
        }
    }
}



fn main() {
    let n= Vect::new([String::from("value")]);
    let mut b = Vect::new([0, 1, 54]);
    let a = n.get_value(0);
    let m = b.get_mut_value(0);
    let mut vec = vec![1, 3, 4];
}
