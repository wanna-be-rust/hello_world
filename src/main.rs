fn main() {
    println!("Hello, world!");
    try_mutable();

    let result = add(2, 3);
    println!("The result is of : {}", result);

    print_number(3);
    print_number_while(4);

    let person = Person::new("ElSalvo", 27);
    person.greet();

    let mut shape = Shape::Circle(2.0);
    println!("Area of the shape: {}", shape.area());
    shape = Shape::Square(2.0);
    println!("Area of the shape: {}", shape.area());
}

fn try_mutable() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

// Make a function that print the number from 0 to input number
fn print_number(num: i32) {
    for i in 0..num {
        println!("The number is: {}", i);
    }
}

// Make the same function but with while loop
fn print_number_while(num: i32) {
    let mut i = 0;
    while i < num {
        println!("The number is: {}", i);
        i += 1;
    }
}

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Person {
        Person {
            name: name.to_string(),
            age,
        }
    }

    fn greet(&self) {
        println!("Hello, my name is {} and I am {} years old.", self.name, self.age);
    }
}

trait Greet {
    fn greet_different(&self);
}

impl Greet for Person {
    fn greet_different(&self) {
        println!("Hello, {} is my name and I am {} years old.", self.name, self.age);
    }
}

enum Shape {
    Circle(f64),
    Square(f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match *self {
            Shape::Circle(ref radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(ref side) => side * side,
        }
    }
}
