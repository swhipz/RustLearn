use std::env;

fn main() {
    let huge = 2.5;
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // Attempt to parse the second argument as a f64
        match args[1].parse::<f64>() {
            Ok(value) => {
                if value == huge {
                    println!("Haha nice, huge!");
                } else {
                    println!("Not huge enough!");
                }
            }
            Err(_) => {
                println!("Please provide a valid number!");
            }
        }
    } else {
        println!("Please provide an argument!");
    }
} 