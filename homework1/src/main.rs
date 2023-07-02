trait Fizzbuzz {
    fn fizzbuzz(&self);
}

impl Fizzbuzz for i32 {
    fn fizzbuzz(&self) {
        if self % 5 == 0 && self % 3 == 0 {
            println!("FizzBuzz");
        } else if self % 3 == 0 {
            println!("Fizz");
        } else if self % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", self);
        }
    }
}


fn main() { 
for i in 1..=100 {
    i.fizzbuzz();
}
}

