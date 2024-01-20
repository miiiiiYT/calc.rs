// calc.rs. A basic commandline calculator written in Rust.
// Copyright (C) 2024 miiiiiyt

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::env;
use std::{io::{self, BufRead}, f64::NAN};
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
    SquareRoot,
}

struct Expression {
    first: f64,
    operation: Operation,
    second: f64,
}

enum TextMode {
    Silly,
    Serious,
}

struct Messages {
    license: &'static str,
    welcome: &'static str,
    goodbye: &'static str,
    error: &'static str,
    info: &'static str,
}

fn main() {
    let cli_args: Vec<String> = env::args().collect();

    let mode = if cli_args.contains(&String::from("--serious")) {
        TextMode::Serious
    } else if cli_args.contains(&String::from("--silly")) {
        TextMode::Silly
    } else {
        TextMode::Serious
    };

    let messages = get_messages(mode);

    if !(cli_args.contains(&String::from("--suppress-notice")) || cli_args.contains(&String::from("-n"))) {
        println!("{}\n", messages.license);
    }
    println!("{}",messages.welcome);

    loop {
        print!("calc.rs $> ");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");
        let input = read_input();

        if input == "" {
            continue;
        } else if input == "exit" {
            println!("{}",messages.goodbye);
            println!();
            break;
        } else if input == "info" {
            println!("{}",messages.info)
        } else {
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
                println!("{}",messages.error);
            }
        }
    }
}

fn get_messages(mode: TextMode) -> Messages {
    match mode {
        TextMode::Silly => Messages {
            license: "calc.rs  Copyright (C) 2024  miiiiiyt
This program comes with ABSOLUTELY NO WARRANTY; for details type `info'.
This is free software, and you are welcome to redistribute it
under certain conditions; type `info' for details.",
            welcome: "Welcome to calc.rs :3\nMade by miiiiiyt.\nPlease note that of right now you can only supply two numbers.\nArguments need to be seperated by spaces.",
            goodbye: "Take care love <3",
            error: "Oopsie :c",
            info: "calc.rs (c) 2024 miiiiiyt\nA basic commandline calculator written in Rust.\nGithub: https://github.com/miiiiiyt/calc.rs\nLicense: https://www.gnu.org/licenses/gpl-3.0\nNow with added fun! :3",
        },
        TextMode::Serious => Messages {
            license: "calc.rs  Copyright (C) 2024  miiiiiyt
This program comes with ABSOLUTELY NO WARRANTY; for details type `info'.
This is free software, and you are welcome to redistribute it
under certain conditions; type `info' for details.",
            welcome: "Welcome to calc.rs.\nMade by miiiiiyt.\nPlease note that of right now you can only supply two numbers.\nArguments need to be seperated by spaces.",
            goodbye: "Goodbye",
            error: "An error occured. Please check your input.\nIf you believe this to be a bug, please report it on the Github.\nType info for more info.",
            info: "calc.rs (c) 2024 miiiiiyt\nA basic commandline calculator written in Rust.\nGithub: https://github.com/miiiiiyt/calc.rs\nLicense: https://www.gnu.org/licenses/gpl-3.0\nNo fun allowed today.\n(see https://github.com/miiiiiYT/calc.rs/tree/master#flags for instructions on how to enable fun)",
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