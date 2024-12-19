use std::env;

fn main() {
    let huge = 2.5;
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "huge" {
        if args[1].parse::<f64>().unwrap_or(0.0) == huge {
            println!("Haha nice, huge!");
        } else {
            println!("Not huge enough!");
        }
    } else {
        println!("Please provide an argument!");
    }
}


