use regex::Regex;
use std::fs;

fn get_adjacent_indices(row: usize, col: usize, max_rows: usize, max_cols: usize) -> [(usize, usize); 8] {
    let pre_row = if row > 0 { row - 1 } else { 0 };
    let next_row = if row < max_rows - 1 { row + 1 } else { row };
    let pre_col = if col > 0 { col - 1 } else { 0 };
    let next_col = if col < max_cols - 1 { col + 1 } else { col };

    return [(pre_row, pre_col), (pre_row, col), (pre_row, next_col), 
            (row, pre_col), (row, next_col),
            (next_row, pre_col), (next_row, col), (next_row, next_col)];
}

fn print_matrix(matrix: &Vec<Vec<i32>>) {
    for row in matrix {
        for &elem in row {
            print!("{}\t", elem);
        }
        println!(); // To start a new line after each row
    }
    println!("-----------")
}

fn print_vector(vector: [(usize, usize); 8]) -> [(usize, usize); 8] {
    for elem in vector {
        print!("({},{})\t", elem.0, elem.1);
    }
    println!(); // To start a new line after each row
    println!("-----------");

    return vector;
}

fn print_vec(vector: &Vec<(usize, usize)>)  {
    for elem in vector {
        print!("({},{})\t", elem.0, elem.1);
    }
    println!(); // To start a new line after each row
    println!("-----------");
}

fn get_and_reset(matrix: &mut Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
    let curr_num = matrix[row][col];

    if col > 0 {
        for c in (col-1)..0 {
            if matrix[row][c] == curr_num {
                matrix[row][c] = 0;
            } else {
                break;
            } 
        }
    }

    for c in col..(matrix[row].len()) {
        if matrix[row][c] == curr_num {
            matrix[row][c] = 0;
        } else {
            break;
        } 
    }

    // print_matrix(matrix);
    return curr_num;
}

fn get_and_reset2(matrix: &mut Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
    let curr_num = matrix[row][col];

    if col > 0 {
        for c in (col-1)..0 {
            if matrix[row][c] == curr_num {
                matrix[row][c] = 0;
            } else {
                break;
            } 
        }
    }

    for c in col..(matrix[row].len()) {
        if matrix[row][c] == curr_num {
            matrix[row][c] = 0;
        } else {
            break;
        } 
    }

    // print_matrix(matrix);
    return if curr_num != 0 { curr_num } else { 1 };
}
pub fn problem1(filename: &str) {
    let file_contents = fs::read_to_string(filename).expect("Error reading file");

    let mut matrix: Vec<Vec<i32>> = Vec::new();
    let num_re = Regex::new(r"(\d+)").unwrap();
    let mut symbol_positions: Vec<(usize, usize)> = Vec::new();
    let mut curr_line: usize = 0;

    let mut max_cols: usize = 0;

    for line in file_contents.lines() {
        max_cols = line.chars().count();
        let mut vec = vec![0; max_cols];

        for num_match in num_re.find_iter(line) {
            let start = num_match.start();
            let end = num_match.end();
            let num = line
                .get(start..end)
                .unwrap()
                .parse::<i32>()
                .unwrap();

            for i in start..end {
                // For all the char positions assign the same number
                // so that when we look at symbol then we can get the number.
                vec[i] = num;
            }
        }

        for (index, c) in line.char_indices() {
            if !c.is_numeric() && c != '.' {
                symbol_positions.push((curr_line, index));
            }
        }

        curr_line += 1;
        matrix.push(vec);
    }

    //print_vec(&symbol_positions);

    let max_rows: usize = matrix.len();
    let mut sum = 0;
    for (row, col) in symbol_positions {
        sum = get_adjacent_indices(row, col, max_rows, max_cols)
            .iter()
            .fold(sum, |agg, (r,c)| agg + get_and_reset(&mut matrix, *r, *c));
        //print_matrix(&matrix);
    }
    println!("Sum is {}", sum);

}

pub fn problem2(filename: &str) {
    let file_contents = fs::read_to_string(filename).expect("Error reading file");

    let mut matrix: Vec<Vec<i32>> = Vec::new();
    let num_re = Regex::new(r"(\d+)").unwrap();
    let mut symbol_positions: Vec<(usize, usize)> = Vec::new();
    let mut curr_line: usize = 0;

    let mut max_cols: usize = 0;

    for line in file_contents.lines() {
        max_cols = line.chars().count();
        let mut vec = vec![0; max_cols];

        for num_match in num_re.find_iter(line) {
            let start = num_match.start();
            let end = num_match.end();
            let num = line
                .get(start..end)
                .unwrap()
                .parse::<i32>()
                .unwrap();

            for i in start..end {
                // For all the char positions assign the same number
                // so that when we look at symbol then we can get the number.
                vec[i] = num;
            }
        }

        for (index, c) in line.char_indices() {
            if c == '*' {
                symbol_positions.push((curr_line, index));
            }
        }

        curr_line += 1;
        matrix.push(vec);
    }

    //print_vec(&symbol_positions);

    let max_rows: usize = matrix.len();
    let mut sum = 0;
    for (row, col) in symbol_positions {
        let adjacent_indices = get_adjacent_indices(row, col, max_rows, max_cols);
        let mut cloned_matrix = matrix.clone();

        let count_nums = adjacent_indices
            .iter()
            .filter_map(
                |(r, c)| {
                    let num = get_and_reset(&mut cloned_matrix, *r, *c);
                    if num != 0 { 
                        return Some(num);
                    } else { 
                        return None;
                    }
            })
            .count();

        // println!("{}", count_nums);

        if count_nums <= 1 {
            continue;
        }

        sum += adjacent_indices
            .iter()
            .fold(1, |agg, (r,c)| agg * get_and_reset2(&mut matrix, *r, *c));
        //print_matrix(&matrix);
    }
    println!("Sum is {}", sum);

}


