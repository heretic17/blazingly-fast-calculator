use std::io;

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("Enter the first number:");
    io::stdin().read_line(&mut num1).unwrap();

    println!("Enter the second number:");
    io::stdin().read_line(&mut num2).unwrap();

    let num1: u64 = match num1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            print!("Error");
            return;
        }
    };

    let num2: u64 = match num2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            print!("Error");
            return;
        }
    };

    println!("Select an operator:");
    println!("1. Add");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Divide");

    let mut operator = String::new();

    io::stdin().read_line(&mut operator).expect("Failed to read line");

    let parse_operator: i32 = match  operator.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            print!("Error");
            return;
        }
    };

    match parse_operator {
        1 => {
            println!("Result: {}", num1 + num2)
        }
        2 => {
            println!("Result: {}", num1 - num2)
        }
        3 => {
            println!("Result: {}", num1 * num2)
        }
        4 => {
            if num2 != 0 {
                println!("Result: {}", num1 / num2);
            }
            else {
                println!("Error: Division by zero!");
            }
        }
        _ => {
            println!("Error!")
        }
    }
}
