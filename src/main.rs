use core::fmt;
use std::{io::{self, BufRead}, f64::NAN};
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
    SquareRoot,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

struct Expression {
    first: f64,
    operation: Operation,
    second: f64,
}

fn main() {
    println!("Welcome to calc.rs :3\nMade by miiiiiyt.\nPlease note that of right now you can only supply two numbers.\nArguments need to be seperated by spaces.");

    loop {
        print!("calc.rs $> ");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");
        let input = read_input();

        if input == "" {
            continue;
        }
        
        if input == "exit" {
            println!("Goodbye! <3");
            println!();
            break;
        }

        let mut args = process_input(input);
        args.resize(3, "".to_string());

        let expression = create_expression(args);

        if expression.is_some() {
            let unwrapped = expression.unwrap();        
            let number1 = unwrapped.first;
            let number2 = unwrapped.second;
            let result: f64 = match unwrapped.operation {
                Operation::Add => number1 + number2,
                Operation::Subtract => number1 - number2,
                Operation::Multiply => number1 * number2,
                Operation::Divide => number1 / number2,
                Operation::Exponent => number1.powf(number2),
                Operation::SquareRoot => number1.sqrt(),
            };
            println!("{}", result);
        } else {
            println!("Error occured :c");
        }
    }
}

fn read_input() -> String {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    return line
}

fn process_input(input: String) -> Vec<String> {
    let args = input.split_whitespace();
    let mut arg_list: Vec<String> = Vec::new();
    for arg in args {
        arg_list.push(arg.to_string())
    }
    return arg_list
}

fn create_expression(args: Vec<String>) -> Option<Expression> {
    if args.len() > 3 {
        None
    } else {
        let operation: Option<Operation> = match args.iter().nth(1).unwrap().chars().nth(0) {
            Some('+') => Some(Operation::Add),
            Some('-') => Some(Operation::Subtract),
            Some('*') => Some(Operation::Multiply),
            Some('/') => Some(Operation::Divide),
            Some('^') => Some(Operation::Exponent),
            Some('#') => Some(Operation::SquareRoot),
            _ => None,
        };

        let number1 = args.get(0)?.parse::<f64>().unwrap_or(NAN);
        let number2 = args.get(2)?.parse::<f64>().unwrap_or(NAN);

        if number1.is_nan() || number2.is_nan() {
            return None
        }

        if operation.is_some() {
            Some(Expression { first: number1, operation: operation.unwrap(), second: number2 })
        } else {
            None
        }  
    }
    
    
}