use std::fs;
use std::collections::HashMap;

fn print_vec(vector: &Vec<SourceDestinationRange>)  {
    for elem in vector {
        print!("{} | {} | {} \t", elem.source_range_start, elem.destination_range_start, elem.range_len);
    }
    println!(); // To start a new line after each row
    println!("-----------");
}

struct SourceDestinationRange {
    source_range_start: u64,
    destination_range_start: u64,
    range_len: u64
}

pub fn problem1(filename: &str) {
    let file_contents = fs::read_to_string(filename).expect("Error reading file");

    let mut lines = file_contents.lines();

    let seeds_line = lines.next().unwrap();

    let nums = seeds_line[("seeds:").len()..].trim(); // Skip the `seeds:`
    let seeds: Vec<u64> = nums
        .split(" ")
        .map(|num_str| num_str.parse::<u64>().unwrap())
        .collect();

    lines.next(); // Skip the empty line

    let mut source_destination_pairs: Vec<(&str, &str)> = vec![];
    let mut source_destination_mapping: HashMap<&str, Vec<SourceDestinationRange>> = HashMap::new();
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
            source_destination_mapping.insert(source.unwrap(), vec![]);

            // Go to next line as we have extracted the source and destination
            continue;
        }

        // Make sure we have both source and destination figured by the time we get here
        assert!(source != None && destination != None, "Source and Destination should be available");

        let source_to_destination_nums: Vec<u64> = line.trim().split(" ").map(|n| n.parse::<u64>().unwrap()).collect();

        let destination_range_start = source_to_destination_nums[0];
        let source_range_start = source_to_destination_nums[1];
        let range_len = source_to_destination_nums[2];

        source_destination_mapping
            .get_mut(source.unwrap())
            .unwrap()
            .push(SourceDestinationRange { 
                source_range_start: source_range_start,
                destination_range_start: destination_range_start,
                range_len: range_len
            });
    }

    let mut min_location: u64 = u64::MAX;
    for seed in &seeds {
        print!("START");

        let mut prev_value = *seed;
        for &(source, _destination) in &source_destination_pairs {
            print!("-> {}: {} ", source, prev_value);

            // if source == "fertilizer" {
            //     print_vec(&source_destination_mapping[source]);
            // }

            for SourceDestinationRange { source_range_start, destination_range_start, range_len } in &source_destination_mapping[source] {
                if prev_value >= *source_range_start && prev_value < (*source_range_start + *range_len) {
                    // if source == "fertilizer" {
                    //     println!(" Picked ({},{},{})", source_range_start, destination_range_start, range_len);
                    // }
                    prev_value = *destination_range_start + (prev_value - *source_range_start);
                    break;
                }
            }
        }

        println!();
        min_location = min_location.min(prev_value);
    }

    println!("Min location: {}", min_location);
}

pub fn problem2(filename: &str) {
    let file_contents = fs::read_to_string(filename).expect("Error reading file");

    let mut lines = file_contents.lines();

    let seeds_line = lines.next().unwrap();

    let nums = seeds_line[("seeds:").len()..].trim(); // Skip the `seeds:`
    let seeds: Vec<u64> = nums
        .split(" ")
        .map(|num_str| num_str.parse::<u64>().unwrap())
        .collect();


    lines.next(); // Skip the empty line

    let mut source_destination_pairs: Vec<(&str, &str)> = vec![];
    let mut source_destination_mapping: HashMap<&str, Vec<SourceDestinationRange>> = HashMap::new();
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
            source_destination_mapping.insert(source.unwrap(), vec![]);

            // Go to next line as we have extracted the source and destination
            continue;
        }

        // Make sure we have both source and destination figured by the time we get here
        assert!(source != None && destination != None, "Source and Destination should be available");

        let source_to_destination_nums: Vec<u64> = line.trim().split(" ").map(|n| n.parse::<u64>().unwrap()).collect();

        let destination_range_start = source_to_destination_nums[0];
        let source_range_start = source_to_destination_nums[1];
        let range_len = source_to_destination_nums[2];

        source_destination_mapping
            .get_mut(source.unwrap())
            .unwrap()
            .push(SourceDestinationRange { 
                source_range_start: source_range_start,
                destination_range_start: destination_range_start,
                range_len: range_len
            });
    }

    let mut min_location: u64 = u64::MAX;

    let seed_pairs: Vec<(u64, u64)> = seeds
        .chunks_exact(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect();
    for &(seed_start, seed_len) in &seed_pairs{
        for seed in seed_start..(seed_start+seed_len) {
            // print!("START");

            let mut prev_value = seed;
            for &(source, _destination) in &source_destination_pairs {
                // print!("-> {}: {} ", source, prev_value);

                // if source == "fertilizer" {
                //     print_vec(&source_destination_mapping[source]);
                // }

                for SourceDestinationRange { source_range_start, destination_range_start, range_len } in &source_destination_mapping[source] {
                    if prev_value >= *source_range_start && prev_value < (*source_range_start + *range_len) {
                        // if source == "fertilizer" {
                        //     println!(" Picked ({},{},{})", source_range_start, destination_range_start, range_len);
                        // }
                        prev_value = *destination_range_start + (prev_value - *source_range_start);
                        break;
                    }
                }
            }

            // println!();
            min_location = min_location.min(prev_value);
        }
    }

    println!("Min location: {}", min_location);
}

