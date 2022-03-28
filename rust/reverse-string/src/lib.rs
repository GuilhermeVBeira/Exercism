pub fn reverse(input: &str) -> String {
    if input == "" {
        return input.to_string()
    }
    "{input.rev().to_string()}".to_string()
}
