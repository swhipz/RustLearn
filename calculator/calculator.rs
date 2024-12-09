// guess i need struct
struct Calculator {

}

impl Calculator {
    // instance
    fn new() -> Self {
        Calculator {
            // field
        }
    }

    // calc methods
    fn add(&self, num1: f64, num2: f64) -> f64 {
        num1 + num2
    }

    fn subtract(&self, num1: f64, num2: f64) -> f64 {
        num1 - num2
    }

    fn multiply(&self, num1: f64, num2: f64) -> f64 {
        num1 * num2
    }

    fn divide(&self, num1: f64, num2: f64) -> f64 {
        if num2 == 0.0 {
            panic!("Cannot divide by zero!");
        }
        num1 / num2
    }

    // input
    fn calculate(&self) {
        use std::io;

        loop {
            println!("Enter operation (add, subtract, multiply, divide) or 'exit' to quit:");
            let mut operation = String::new();
            io::stdin().read_line(&mut operation).expect("Failed to read line");
            let operation = operation.trim();

            if operation == "exit" {
                break;
            }

            println!("Enter two numbers separated by space:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let numbers: Vec<&str> = input.trim().split_whitespace().collect();

            if numbers.len() != 2 {
                println!("Please enter exactly two numbers.");
                continue;
            }

            let num1: f64 = match numbers[0].parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Invalid number: {}", numbers[0]);
                    continue;
                }
            };

            let num2: f64 = match numbers[1].parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Invalid number: {}", numbers[1]);
                    continue;
                }
            };

            let result = match operation {
                "add" => self.add(num1, num2),
                "subtract" => self.subtract(num1, num2),
                "multiply" => self.multiply(num1, num2),
                "divide" => self.divide(num1, num2),
                _ => {
                    println!("Unknown operation: {}", operation);
                    continue;
                }
            };

            println!("Result: {}", result);
        }
    }
}

// calculate run
fn run_calculator() {
    let calculator = Calculator::new();
    calculator.calculate(); 
}

fn main() {
    run_calculator();
}

