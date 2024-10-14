use std::io::{self, stdin};

fn main() {
    println!("{}", calculate().to_string());
}

fn enter_num() -> Result<f64, String> {
    println!("Enter number: ");
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    input = input.trim().to_string();
    
    match input.parse::<f64>() {
        Ok(num) => Ok(num),
        Err(_) => Err("Please enter a valid floating-point number.".to_string()),
    }
}

fn calculate() -> f64 {
    let num1: f64;
    let num2: f64;
    let mut operator = String::new();

    loop {
        match enter_num() {
            Ok(result) => {
                num1 = result;
                break;
            },
            Err(e) => {
                eprintln!("{}", e);
                println!("Please try again.");
            }
        }    
    }

    loop {
        match enter_num() {
            Ok(result) => {
                num2 = result;
                break;
            },
            Err(e) => {
                eprintln!("{}", e);
                println!("Please try again.");
            }
        }    
    }

    loop {
        println!("Enter operator: (+ | - | * | /)");
        stdin().read_line(&mut operator).expect("Failed to read line.");

        operator = operator.trim().to_string();

        match operator.as_str() {
            "+" | "-" | "*" | "/" => break,
            _ => {
                eprintln!("Invalid operator. Please try again.");
                println!("Valid operators are: +, -, *, /");
            }
        }        
    }

    match operator.as_str() {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 != 0.0 {
                num1 / num2
            } else {
                eprintln!("Cannot divide by zero!");
                0.0
            }
        },
        _ => unreachable!(),
    }
}

