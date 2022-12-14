pub fn reverse(input: &str) -> String {
    let mut output = String::new();
    for c in String::from(input).chars().rev() {
        output.push(c);
    }
    return output;
}
