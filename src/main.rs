unsafe fn main() {
}

fn arrays() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("First Array Element {}", array[0]);
    array[0] = 24;
    println!("First Array Element {}", array[0]);
}

fn push_string() {
    let mut string = String::from("Hello");
    string.push_str(", world!");
    println!("{}", string);
}

fn simple_method_call() {
    let mut x= 8; // mut keyword for the changeability of the variable
    println!("The result of the addition is: {}", add(1, x));
    x = 10;
    println!("The result of the addition is: {}", add(1, x))
}

fn add(first: i8, second: i8) -> i8 {
    return first + second;
}