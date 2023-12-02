use regex::Regex;
use std::{collections::HashMap, fs};

pub fn problem1(filename: &str) {
    let file_contents = fs::read_to_string(filename).expect("Error reading file");

    let max_count = HashMap::<&str, i32>::from([("red", 12), ("blue", 14), ("green", 13)]);

    let mut sum = 0;
    let parse_cubes_re = Regex::new(r"(\d+)\s+(red|blue|green)(;*)").unwrap();
    let parse_game_re = Regex::new(r"Game (\d+):").unwrap();
    for line in file_contents.lines() {
        // print!("{}", line);
        let mut cube_count = HashMap::<&str, i32>::from([("red", 0), ("blue", 0), ("green", 0)]);
        let mut is_valid = true;

        for (_, [num, color, separator]) in parse_cubes_re.captures_iter(line).map(|c| c.extract())
        {
            let color_num = cube_count.get_mut(color).unwrap();
            *color_num += num.parse::<i32>().unwrap();

            if *color_num > *max_count.get(color).unwrap() {
                // println!("----- IN-VALID");
                is_valid = false;
                break;
            }

            if separator == ";" {
                cube_count = HashMap::<&str, i32>::from([("red", 0), ("blue", 0), ("green", 0)]);
            }
        }

        if !is_valid {
            // If the cubs were not in the range then continue to next line
            continue;
        }

        // println!("----- VALID");
        let (_, [game_num]) = parse_game_re.captures(line).unwrap().extract();

        sum += game_num.parse::<i32>().unwrap();
    }

    println!("Sum is: {}", sum);
}

pub fn problem2(filename: &str) {
    let file_contents = fs::read_to_string(filename).expect("Error reading file");

    let parse_cubes_re = Regex::new(r"(\d+)\s+(red|blue|green)(;*)").unwrap();

    let mut sum = 0;

    for line in file_contents.lines() {
        let mut cube_max_count = HashMap::<&str, i32>::from([("red", 0), ("blue", 0), ("green", 0)]);
        let mut cube_count = HashMap::<&str, i32>::from([("red", 0), ("blue", 0), ("green", 0)]);

        let mut update_max_count = |cube_count: &mut HashMap<&str, i32>| {
            // Iterate over the keys for cube_min_count and set each color to min
            for (color, val) in cube_max_count.iter_mut() {
                let cube_count_container = cube_count.get(color).unwrap();

                *val = (*val).max(*cube_count_container);
            }

            *cube_count = HashMap::<&str, i32>::from([("red", 0), ("blue", 0), ("green", 0)]);

        };

        for (_, [num, color, separator]) in parse_cubes_re.captures_iter(line).map(|c| c.extract())
        {
            let color_num = cube_count.get_mut(color).unwrap();
            *color_num += num.parse::<i32>().unwrap();

            if separator == ";" {
                update_max_count(&mut cube_count);
            }
        }

        update_max_count(&mut cube_count);

        sum += cube_max_count.values().fold(1, |acc, val| acc * val);
    }

    println!("Sum is: {}", sum);
}