//Вектор и хэшмап 

use std::{rc::Rc, cell::RefCell};

fn main() {
    let a = String::from("value");
    let A = A {
        vec: Rc::new(RefCell::new(a))
    };
}

struct A {
    vec: Rc<RefCell<String>>
}