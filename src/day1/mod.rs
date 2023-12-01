use std::fs;
use std::io::Write;
use std::collections::{self, HashMap};

pub fn problem1(filename: &str) {

    let file_contents = fs::read_to_string(filename).expect("Error reading file");

    let mut sum = 0;
    for line in file_contents.lines()  {
        let mut firstNum: Option<char> = None;
        let mut lastNum: Option<char> = None;

        for c in line.chars() {
            if c.is_numeric() {
                match (firstNum, lastNum) {
                    (None, _) => (firstNum, lastNum) = (Some(c), Some(c)),
                    _ => lastNum = Some(c)
                }
            }

        }
        sum += format!("{}{}", firstNum.unwrap(), lastNum.unwrap()).parse::<i32>().expect("Unable to parse value");
    }

    println!("{}", sum)
}

pub fn problem2(filename: &str) {
    let file_contents = fs::read_to_string(filename).expect("Error reading file");

    // Create mapping of digits
    let digits: HashMap<&'static str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    // Create mapping of digits in reverse
    let mut reverse_digits: HashMap<String, char> = HashMap::new();
    for (key, value) in &digits {
        reverse_digits.insert(key.chars().rev().collect(), *value);
    }
    
    let mut sum = 0;
    for line in file_contents.lines()  {
        let mut first_num: Option<char> = None;
        let mut last_num: Option<char> = None;

        let mut prev_index = usize::MAX;

        // First find regular first digit
        for (index, c) in line.chars().enumerate() {
            if c.is_numeric() {
                prev_index = index;
                first_num = Some(c);
                break;
            }
        }

        // Then go over dictionary and find any substring that matches the digit and store the one with lowest index.
        for (digit, value) in &digits {
            match line.find(digit) {
                Some(index) => if index < prev_index {
                    prev_index = index;
                    first_num = Some(*value);
                    last_num = Some(*value);
                },
                None => {} ,
            }
        }

        // Do the same on the reverse of a string with reverse digit match. The reason I went with this approach
        // was so that if something like threeseveneightseven then if I would have just used find then it would 
        // have given me index of first seven hence I just reversed it. 
        let mut prevRevIndex = usize::MAX;
        let revLine: String = line.chars().rev().collect();
        
        for (index, c) in revLine.chars().enumerate() {
            if c.is_numeric() {
                prevRevIndex = index;
                last_num = Some(c);
                break;
            }
        }

        for (digit, value) in &reverse_digits{
            match revLine.find(digit) {
                Some(index) => if index < prevRevIndex{
                    prevRevIndex = index;
                    last_num = Some(*value);
                },
                None => {} ,
            }
        }
        
        sum += format!("{}{}", first_num.unwrap(), last_num.unwrap()).parse::<i32>().expect("Unable to parse value");
    }

    println!("{}", sum)
}