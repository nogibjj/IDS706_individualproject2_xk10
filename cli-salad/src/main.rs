use clap::Parser;
use cli_salad::create_fruit_salad;

#[derive(Parser)]
struct Opts {
    #[arg(short, long, default_value = "0")]
    number: usize,
}

fn main() {
    let opts: Opts = Opts::parse();

    // Get the number of fruits the user requested
    let num_fruits = opts.number;

    // Create the fruit salad
    let salad = create_fruit_salad(num_fruits);

    // Print the fruit salad in a human-readable format with a count of fruits used
    println!(
        "Created Fruit salad with {} fruits: {:?}",
        num_fruits, salad
    );
}

