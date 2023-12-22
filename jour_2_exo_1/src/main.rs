fn main() {
    let (red_max, green_max, blue_max) = (12, 13, 14);
    let mut sum: i32 = 0;
    // read the input file
    let input = std::fs::read_to_string("input.txt").unwrap();
    for line in input.lines(){
        let mut temp_split: Vec<&str> = line.split(":").collect();
        let (game_id, instances_str) = (temp_split[0], temp_split[1]);
        let instances: Vec<&str> = instances_str.split(";").collect();
        let mut is_possible = true;
        for instance in instances{
            let colors: Vec<&str> = instance.split(",").collect();
            for color in colors{
                temp_split = color.split_whitespace().collect();
                let (value, color_name) = (temp_split[0], temp_split[1]);

                if (color_name == "red" && value.parse::<i32>().unwrap() > red_max)
                || (color_name == "green" && value.parse::<i32>().unwrap() > green_max)
                || (color_name == "blue" && value.parse::<i32>().unwrap() > blue_max){
                    println!("Game {} is not possible", game_id);
                    is_possible = false;
                }
            }
        }
        if is_possible{
            println!("Game {} is possible", game_id);
            sum += game_id.split_whitespace().collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        }
    }
    println!("Sum of possible games: {}", sum);
}