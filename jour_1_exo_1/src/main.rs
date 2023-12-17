fn main() {
    let string_numbers = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    // read the input file
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut sum = 0;
    for line in input.lines() {
        print!("{} ", line);

        let mut number = 0;

        // search for the first numeric character or the first string number
        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                number += c.to_digit(10).unwrap() * 10;
                break;
            } 
            else {
                for (j, string_number) in string_numbers.iter().enumerate() {
                    
                    if line[i..].starts_with(string_number) {
                        number += (j + 1) as u32 * 10;
                    }
                }
                if number > 0 {
                    break;
                }
            }
        }

        // search for the last numeric character or the last string number
        for (i, c) in line.chars().rev().enumerate() {
            if c.is_numeric() {
                number += c.to_digit(10).unwrap();
                break;
            } 
            else {
                for (j, string_number) in string_numbers.iter().enumerate() {
                    if line[..line.len() - i].ends_with(string_number) {
                        number += j as u32 + 1;
                    }
                }
                if number % 10 > 0 {
                    break;
                }
            }
        }
        println!("= {} ", number);
        sum += number;
    }
    println!("sum: {}", sum);
}
