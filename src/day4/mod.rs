use std::fs;
use regex::Regex;
use std::collections::HashSet;

pub fn problem1(filename: &str) {

    let file_contents = fs::read_to_string(filename).expect("Error reading file");

    let num_re = Regex::new(r"(\d+)").unwrap();

    let mut sum = 0;

    for line in file_contents.lines() {
        let num_strs: Vec<&str> = line.split(":").collect();

        let split_nums: Vec<&str> = num_strs[1].split("|").collect();
        let win_nums = split_nums[0];
        let curr_nums = split_nums[1];

        let set = num_re
            .captures_iter(curr_nums)
            .map(|c| c.extract())
            .fold(HashSet::<i32>::new(), |mut set, (_, [num])|  {
                set.insert(num.parse::<i32>().unwrap()); return set
            });

        let exponent = num_re
            .captures_iter(win_nums)
            .map(|c| c.extract())
            .filter_map(|(_, [c])| {
                let num = c.parse::<i32>().unwrap();
                if set.contains(&num) {
                    Some(num)
                } else {
                    None
                }
            }).count();

        
        if exponent > 0 {
            sum += 2i32.pow((exponent as u32) - 1);
        }
    }

    println!("{}", sum);
}

fn print_vec(vector: &Vec<u32>)  {
    for elem in vector {
        print!("{}\t", elem);
    }
    println!(); // To start a new line after each row
    println!("-----------");
}
pub fn problem2(filename: &str) {

    let file_contents = fs::read_to_string(filename).expect("Error reading file");

    let num_re = Regex::new(r"(\d+)").unwrap();

    let mut winning_count: Vec<u32> = Vec::new();


    for line in file_contents.lines() {
        let num_strs: Vec<&str> = line.split(":").collect();

        let split_nums: Vec<&str> = num_strs[1].split("|").collect();
        let win_nums = split_nums[0];
        let curr_nums = split_nums[1];

        let set = num_re
            .captures_iter(curr_nums)
            .map(|c| c.extract())
            .fold(HashSet::<i32>::new(), |mut set, (_, [num])|  {
                set.insert(num.parse::<i32>().unwrap()); return set
            });

        let cnt = num_re
            .captures_iter(win_nums)
            .map(|c| c.extract())
            .filter_map(|(_, [c])| {
                let num = c.parse::<i32>().unwrap();
                if set.contains(&num) {
                    Some(num)
                } else {
                    None
                }
            }).count();

        winning_count.push(cnt as u32);
    }

    let mut total_cards = vec![1u32; winning_count.len()];

    for (index, win_cnt) in winning_count.iter().enumerate() {
        if *win_cnt > 0 {
            for i in (index+1)..(index+(*win_cnt as usize)+1) {
                total_cards[i] += total_cards[index];
            }
        }

        //print_vec(&total_cards);
    }

    let sum = total_cards.iter().fold(0, |agg, val| agg + val);
    println!("Sum {}", sum);
}