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

fn main() {
    
}
