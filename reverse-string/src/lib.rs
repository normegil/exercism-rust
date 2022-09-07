pub fn reverse(input: &str) -> String {
    let mut output = String::new();
    println!("{}", String::from(input));
    println!("{:?}", String::from(input).chars());
    for c in String::from(input).chars().rev() {
        output.push(c);
    }
    return output;
}
