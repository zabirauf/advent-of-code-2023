use std::fs;
use std::collections::{ HashMap };
use std::iter::zip;


pub fn problem1(filename: &str) {
    let file_contents = fs::read_to_string(filename).expect("Error reading file");

    let mut lines = file_contents.lines();

    let seeds_line = lines.next().unwrap();

    let nums = seeds_line[("seeds:").len()..].trim(); // Skip the `seeds:`
    let seeds: Vec<u32> = nums
        .split(" ")
        .map(|num_str| num_str.parse::<u32>().unwrap())
        .collect();

    lines.next(); // Skip the empty line

    let mut source_destination_pairs: Vec<(&str, &str)> = vec![];
    let mut source_destination_mapping: HashMap<&str, HashMap<u32, u32>> = HashMap::new();
    let mut source: Option<&str> = None;
    let mut destination: Option<&str> = None;

    for line in lines {
        if line.trim() == "" {
            source = None;
            destination = None;

            // Go to the next line as nothing to parse here
            continue;
        }

        if source == None && destination == None {
            let split_line: Vec<&str> = line[0..(line.len() - "map:".len())].trim().split("-to-").collect();

            source = Some(split_line[0]);
            destination = Some(split_line[1]);

            source_destination_pairs.push((source.unwrap(), destination.unwrap()));
            source_destination_mapping.insert(source.unwrap(), HashMap::new());

            // Go to next line as we have extracted the source and destination
            continue;
        }

        // Make sure we have both source and destination figured by the time we get here
        assert!(source != None && destination != None, "Source and Destination should be available");

        let source_to_destination_nums: Vec<u32> = line.trim().split(" ").map(|n| n.parse::<u32>().unwrap()).collect();

        let destination_range_start = source_to_destination_nums[0];
        let source_range_start = source_to_destination_nums[1];
        let range_len = source_to_destination_nums[2];

        for (k, v) in (source_range_start..(source_range_start+range_len)).zip((destination_range_start..(destination_range_start+range_len))) {
            source_destination_mapping
                .get_mut(source.unwrap())
                .unwrap()
                .insert(k, v);
        }
    }

    let mut min_location: u32 = u32::MAX;
    for seed in &seeds {
        print!("START");

        let mut prev_value = seed;
        for &(source, _destination) in &source_destination_pairs {
            print!("-> {}: {} ", source, *prev_value);
            let val = source_destination_mapping[source].get(prev_value);

            match val {
               Some(inner_val) => prev_value = inner_val,
               None => {} 
            }
        }

        println!();

        min_location = min_location.min(*prev_value);
    }

    println!("Min location: {}", min_location);
}

