use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;

use error::Day1Error;
use error::SplitIntError;

mod error;

// From https://doc.rust-lang.org/rust-by-example/error/result/enter_question_mark.html

// Split the string line into a pair of integers. Throw a ParseIntError error if not parsed
fn split_int_pair(in_string: &str) -> Result<(i32, i32), Day1Error> {
    let mut splitter = in_string.splitn(2, "   ");
    let first = splitter.next();
    let first = first
        .ok_or(SplitIntError::FirstSplitError)?
        .parse::<i32>()
        .map_err(|e| Day1Error::Parse {
            got: first.map(|s| s.to_string()),
            source: e,
        })?;
    let second = splitter.next();
    let second = second
        .ok_or(SplitIntError::SecondSplitError)?
        .parse::<i32>()
        .map_err(|e| Day1Error::Parse {
            got: second.map(|s| s.to_string()),
            source: e,
        })?;
    Ok((first, second))
}

// Build a map of integers
fn build_integer_map(int_vector: &[i32]) -> HashMap<i32, i32> {
    let mut value_count_map = HashMap::new();

    for int_value_ref in int_vector.iter() {
        value_count_map
            .entry(*int_value_ref)
            .and_modify(|x| *x += 1)
            .or_insert(0);
    }

    value_count_map
}

fn main() {
    let path = env::current_dir();

    let path = match path {
        Ok(path) => path,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => panic!("Path not found"),
            io::ErrorKind::PermissionDenied => panic!("Permission denied"),
            _ => panic!("Error: {:?}", e),
        },
    };

    println!("Loading data from dir {}", path.display());

    // Parse data file

    let contents =
        fs::read_to_string("data/input").expect("Should have been able to read the file");

    // Split into lines
    let lines = contents.split("\n");

    let mut left_column_values: Vec<i32> = Vec::new();
    let mut right_column_values: Vec<i32> = Vec::new();

    for line in lines {
        match split_int_pair(line) {
            Ok((left_column_value, right_column_value)) => {
                left_column_values.push(left_column_value);
                right_column_values.push(right_column_value);
            }
            Err(e) => println!("Error: {}", e),
        }
    }
    // Ensure line count is equal for both columns
    assert_eq!(left_column_values.len(), right_column_values.len());

    // Sort values
    left_column_values.sort();
    right_column_values.sort();

    // Calculate total distance
    let left_iter = left_column_values.iter();
    let right_iter = right_column_values.iter();
    let total_distance =
        left_iter
            .zip(right_iter)
            .fold(0, |mut total_distance, (left_val, right_val)| {
                let distance = (right_val - left_val).abs();
                total_distance += distance;
                println!("Sorted Values {left_val},{right_val} distance {distance}");
                total_distance
            });

    println!("Total distance {total_distance}");

    // Part 2 - Calculate similarity

    let right_count_map = build_integer_map(&right_column_values);
    let total_similarity = left_column_values
        .iter()
        .filter_map(|left_val| {
            right_count_map
                .get(left_val)
                .map(|right_count| (left_val, right_count))
        })
        .fold(0, |mut total_similarity, (left_val, right_count)| {
            total_similarity += left_val * right_count;
            total_similarity
        });

    println!("Total similarity {total_similarity}");
}
