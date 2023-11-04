use rusqlite::{params, Connection, Error, Row};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

#[allow(dead_code)]
const LOG_FILE: &str = "query_log.md";

pub fn log_query(query: &str, log_file: &str) {
    if let Ok(mut file) = OpenOptions::new().append(true).create(true).open(log_file) {
        if let Err(err) = writeln!(file, "```sql\n{}\n```\n", query) {
            eprintln!("Error writing to log file: {:?}", err);
        }
    } else {
        eprintln!("Error opening log file for writing.");
    }
}

#[derive(Debug)]
pub struct Drink {
    country: String,
    beer_servings: i32,
    spirit_servings: i32,
    wine_servings: i32,
    total_litres_of_pure_alcohol: f32,
}

// Function to extract data from a URL and save to a file
pub fn extract(url: &str, file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let content = reqwest::blocking::get(url)?.bytes()?;
    let mut file = File::create(file_path)?;
    file.write_all(&content)?;
    Ok(file_path.to_string())
}

// Function to transform and load CSV data into an SQLite database
pub fn transform_load(dataset: &str) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("DrinksDB.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Drinks (
             country TEXT,
             beer_servings INTEGER,
             spirit_servings INTEGER,
             wine_servings INTEGER,
             total_litres_of_pure_alcohol REAL
         )",
        params![],
    )?;

    let file = File::open(dataset)?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;
        let drink = Drink {
            country: record[0].to_string(),
            beer_servings: record[1].parse()?,
            spirit_servings: record[2].parse()?,
            wine_servings: record[3].parse()?,
            total_litres_of_pure_alcohol: record[4].parse()?,
        };

        conn.execute(
            "INSERT INTO Drinks (country, beer_servings, spirit_servings, wine_servings, total_litres_of_pure_alcohol)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                drink.country,
                drink.beer_servings,
                drink.spirit_servings,
                drink.wine_servings,
                drink.total_litres_of_pure_alcohol,
            ],
        )?;
    }

    Ok(())
}

fn create_drinks_from_row(row: &Row) -> Result<Drink, Error> {
    Ok(Drink {
        country: row.get(0)?,
        beer_servings: row.get(1)?,
        spirit_servings: row.get(2)?,
        wine_servings: row.get(3)?,
        total_litres_of_pure_alcohol: row.get(4)?,
    })
}

pub fn query(query_string: &str) -> Result<Vec<Drink>, rusqlite::Error> {
    let conn = Connection::open("DrinksDB.db")?;
    let mut stmt = conn.prepare(query_string)?;

    if query_string
        .trim_start()
        .to_uppercase()
        .starts_with("SELECT")
    {
        let drinks_iter = stmt.query_map([], create_drinks_from_row)?;

        let mut result = Vec::new();
        for drink in drinks_iter {
            match drink {
                Ok(d) => {
                    result.push(d);
                }
                Err(e) => eprintln!("Error in row: {:?}", e),
            }
        }

        Ok(result)
    } else {
        conn.execute_batch(query_string)?;
        Ok(Vec::new()) // Return an empty vector for non-SELECT queries
    }
}
