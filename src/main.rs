use std::io;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// This function calculates matching Operation enum
fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(first_number, second_number) => return first_number + second_number,
        Operation::Subtract(first_number, second_number) => return first_number - second_number,
        Operation::Multiply(first_number, second_number) => return first_number * second_number,
        Operation::Divide(first_number, second_number) => return first_number / second_number,
    }
}

// This function splits user input word by word and returns a String type vector.
fn input_parser(string: String) -> Vec<String> {
    let modified_string = string.as_str().to_owned() + " ";
    let mut result = Vec::new();
    let mut vessel = String::new();
    let mut space_checker = false;

    for c in modified_string.chars() {
        if !c.is_whitespace() {
            space_checker = false;
            vessel.push(c);
        } else if space_checker {
            continue;
        } else {
            space_checker = true;
            result.push(vessel);
            vessel = String::new();
        }
    }
    return result;
}

fn main() {
    println!("Welcome to the CALCULATOR!");

    // A loop that provides user can calculate how many times user would like
    loop {
        println!("Please enter an operation. example: 5 + 2 | OPERATORS : [ + - / * x ] | To quit the program, enter \"exit\" or \"quit\"");
        let mut user_input = String::new();

        match io::stdin().read_line(&mut user_input) {
            Ok(n) => {
                println!("{} bytes read.", n);
            }
            Err(error) => panic!("An error has been occured. {}", error),
        }

        let expression = input_parser(user_input);
        
        // User quits the program if the input is "exit" or "quit"
        if (&*expression[0] == "exit" || &*expression[0] == "quit") && expression.len() == 1{
            println!("Goodbye.");
            break;
        }
        
        // If user enters a diffrent input from numbers, program sends a warning message to user
        let first_number:f64 = match expression[0].parse::<f64>() {
            Ok(number) => number,
            Err(_e) => {
                println!("Please enter a viable number.");
                continue;
            }
        };

        let operator: &str = &*expression[1];

        let second_number: f64 = match expression[2].parse::<f64>() {
            Ok(number) => number,
            Err(_e) => {
                println!("Please enter a viable number.");
                continue;
            }
        };

        let mut resut_of_operation: f64 = 0.0;

        // The program calculates according to matching operator from user input
        match operator {
            "+" => resut_of_operation = calculate(Operation::Add(first_number, second_number)),
            "-" => resut_of_operation = calculate(Operation::Subtract(first_number, second_number)),
            "*" | "x" | "X" => {
                resut_of_operation = calculate(Operation::Multiply(first_number, second_number))
            }
            // If the dividing number is zero, the program send a warning message to user that numbers can't divided by zero
            "/" => {
                if second_number > 0.0 {
                    resut_of_operation = calculate(Operation::Divide(first_number, second_number));
                } else {
                    println!("{} can not be divided by 0!", first_number);
                }
            }
            _ => {
                println!("Please enter a expression by suggested method. example: 5 + 2");
                continue;
            },
        }

        println!("The result is {}", resut_of_operation);
    }
}
