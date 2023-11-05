// You need to make sure your crate and rusqlite are in your dependencies in Cargo.toml
use rusqlite::{params, Connection, Result};
use your_crate_name::{extract, query, transform_load};

// The test module
#[cfg(test)]
mod tests {
    use super::*;

    // This function is to clear the table before each test run
    fn clear_table() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open("DrinksDB.db")?;
        conn.execute("DELETE FROM Drinks", params![])?;
        Ok(())
    }

    #[test]
    fn test_extract() {
        // Assuming 'test_drinks.csv' is available and correct
        let result = extract("https://raw.githubusercontent.com/fivethirtyeight/data/master/alcohol-consumption/drinks.csv", "test_drinks.csv");
        assert!(result.is_ok(), "Extraction should be successful");
    }

    #[test]
    fn test_transform_load() {
        // Clear the Drinks table before running the transform_load test
        clear_table().expect("Failed to clear Drinks table before test.");

        // Test the transform_load function
        let result = transform_load("test_drinks.csv");
        match result {
            Ok(()) => println!("Transform and load succeeded."),
            Err(e) => {
                eprintln!("Error during loading: {:?}", e);
                panic!("Transform and load failed.");
            }
        }
    }

    #[test]
    fn test_query() {
        // Assuming a correct SQL query for your Drinks table
        let result = query("SELECT * FROM Drinks");
        assert!(result.is_ok(), "Query should be successful");
        if let Ok(drinks) = result {
            assert!(!drinks.is_empty(), "Query should return at least one row");
        }
    }
}
