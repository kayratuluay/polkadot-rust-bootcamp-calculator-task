use std::io;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(first_number, second_number) => return first_number + second_number,
        Operation::Subtract(first_number, second_number) => return first_number - second_number,
        Operation::Multiply(first_number, second_number) => return first_number * second_number,
        Operation::Divide(first_number, second_number) => return first_number / second_number,
    }
}

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
    println!("Please enter an operation. example: 5 + 2");
    let mut user_input = String::new();

    match io::stdin().read_line(&mut user_input) {
        Ok(n) => {
            println!("{} bytes read.", n);
            println!("{}", user_input);
        },
        Err(error) => println!("An error has been occured. {}", error)
    }
    
    let expression = input_parser(user_input);

    let first_number: f64 = expression[0].parse().unwrap();
    let operator: &str = &*expression[1];
    let second_number: f64 = expression[2].parse().unwrap();

    let mut resut_of_operation: f64 = 0.0;

    match operator {
        "+" => resut_of_operation = calculate(Operation::Add(first_number, second_number)),
        "-" => resut_of_operation = calculate(Operation::Subtract(first_number, second_number)),
        "*" | "x" => resut_of_operation = calculate(Operation::Multiply(first_number, second_number)),
        "/" => {
            if second_number > 0.0 {
                resut_of_operation = calculate(Operation::Divide(first_number, second_number));
            } else {
                println!("{} can not be divided by 0!", first_number);
            }
        },
        _ => println!("Please enter a expression by suggested method. example: 5 + 2")
    }

    println!("The result is {}", resut_of_operation);

}
