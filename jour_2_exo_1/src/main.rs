fn main() {
    // read the input file
    let input = std::fs::read_to_string("input.txt").unwrap();
    line = input.lines().next().unwrap();
    println!("{}", line);
}
