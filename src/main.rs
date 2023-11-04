use std::env;
use your_crate_name::{extract, log_query, query, transform_load}; // Ensure you import log_query if it's part of the same crate

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} [action]", args[0]);
        return;
    }

    let action = &args[1];
    match action.as_str() {
        "extract" => {
            let result = extract(
                "https://raw.githubusercontent.com/fivethirtyeight/data/master/alcohol-consumption/drinks.csv",
                "drinks.csv",
            );
            match result {
                Ok(path) => println!("Data extraction completed successfully. Saved to {}", path),
                Err(e) => eprintln!("Error during extraction: {:?}", e),
            }
        }
        "transform_load" => match transform_load("drinks.csv") {
            Ok(()) => println!("Data transformation and loading completed successfully."),
            Err(e) => eprintln!("Error during loading: {:?}", e),
        },
        "query" => {
            if args.len() < 3 {
                println!("Please provide a SQL query string");
                return;
            }
            let query_string = &args[2];
            match query(query_string) {
                Ok(drinks) => {
                    for drink in drinks {
                        println!("{:?}", drink);
                    }
                    log_query(query_string, "query_log.md");
                }
                Err(e) => eprintln!("Error during query execution: {:?}", e),
            }
        }
        _ => {
            println!("Invalid action. Use 'extract', 'transform_load', or 'query'.");
        }
    }
}
