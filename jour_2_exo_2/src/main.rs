fn main() {
    let mut sum: i32 = 0;
    // read the input file
    let input = std::fs::read_to_string("input.txt").unwrap();
    for line in input.lines(){
        let mut temp_split: Vec<&str> = line.split(":").collect();
        let (game_id, instances_str) = (temp_split[0], temp_split[1]);
        let instances: Vec<&str> = instances_str.split(";").collect();
        let (mut min_red, mut min_blue, mut min_green) = (0, 0, 0);
        for instance in instances{
            let colors: Vec<&str> = instance.split(",").collect();
            for color in colors{
                temp_split = color.split_whitespace().collect();
                let (value, color_name) = (temp_split[0].parse::<i32>().unwrap() , temp_split[1]);
                
                match color_name{
                    "red" => {
                        if value > min_red{
                            min_red = value;
                        }
                    },
                    "blue" => {
                        if value > min_blue{
                            min_blue = value;
                        }
                    },
                    "green" => {
                        if value > min_green{
                            min_green = value;
                        }
                    },
                    _ => println!("Error: unknown color"),
                }
            }
        }
        println!("Game {} : {} {} {}", game_id, min_red, min_blue, min_green);
        let game_power = min_red * min_blue * min_green;
        sum += game_power;
    }
    println!("Sum of games' power: {}", sum);
}