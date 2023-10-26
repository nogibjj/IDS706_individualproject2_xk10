#[cfg(test)]
mod tests {
    // Import the create_fruit_salad function
    use cli_salad::create_fruit_salad;

    #[test]
    fn test_create_fruit_salad_size() {
        let number_of_fruits = 5;
        let salad = create_fruit_salad(number_of_fruits);
        assert_eq!(salad.len(), number_of_fruits, "Expected {} fruits in the salad, but got {}", number_of_fruits, salad.len());
    }
}

