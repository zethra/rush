//To Do
//Work on making functions generic

pub fn calculate() {
    let num1: i64 = 5;
    let num2: i64 = 30;
    println!("add: {} \n mult: {} \n div: {} \n sub: {} \n mod: {}", add(num1,num2), mult(num1,num2), div(num1,num2), sub(num1,num2), modulo(num1,num2));
}

fn add(num1: i64, num2: i64) -> i64 {
    num1 + num2
}

fn mult(num1: i64, num2: i64) -> i64 {
   num1 * num2 
}

fn div(num1: i64, num2: i64) -> i64 {
    num1 / num2
}

fn sub(num1: i64, num2: i64) -> i64 {
    num1 - num2
}

fn modulo(num1: i64, num2: i64) -> i64 {
    num1 % num2
}
