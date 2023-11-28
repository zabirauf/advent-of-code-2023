use std::fs;
use std::io::Write;

pub fn problem1(filename: &str, output_filename: &str) {

    let file_contents = fs::read_to_string(filename).expect("Error reading file");

    let mut output_file = fs::OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .open(output_filename)
        .expect("Unable to create/open output file");

    for line in file_contents.lines()  {
        output_file.write_fmt(format_args!("{}\n", line)).expect("Expected output file to be written to");
    }

    println!("Done")
}