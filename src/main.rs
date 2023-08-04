use std::io;

enum Operation {
    Add(f64, f64), // kacak
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
    let mut user_input = String::new();
    match io::stdin().read_line(&mut user_input) {
        Ok(n) => {
            println!("{} bytes read.", n);
            println!("{}", user_input);
        },
        Err(error) => println!("An error has been occured. {}", error),
    }
    
    

    

    let expression = input_parser(user_input);

    println!("{:?}", expression);


}
