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

fn input_parser(mut string: String) -> Vec<String> {
    string.push(' ');
    let mut result = Vec::new();
    let mut vessel = String::new();
    let mut space_checker = false;

    for c in string.chars() {
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

    loop {
        let mut user_input = String::new();

        
        // let operations_display: [String; 4] = [""]

        println!("Please enter a number");
        user_input = get_input();
        let first_num:f64 = user_input.parse().unwrap();

        println!("{}", user_input);
        break;
        
    }


}
