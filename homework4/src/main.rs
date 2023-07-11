use homework4::Vect;

fn main() {
    // пример: new, push, with_capacity
    // адекватного вывода нет(((
    let mut a = Vect::new([String::from("de")]); // new должен знать кол-во элементов массива во время компиляции (плохо) 
    a.push("elment".to_string());
    println!("{:?}", a);
    let mut a: Vect<usize, 3> = Vect::with_capacity(); // Емкость передаётся не в аргументы, а в параметр (плохо) (проблема как и с new)
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
    a.push(3);
    a.push(4);
    a.push(5);
    a.push(6);
    println!("{}", a.remove(3));
    println!("{:?}", a);

}
