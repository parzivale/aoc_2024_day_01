use std::env;
use std::fs;
use std::num::ParseIntError;

// From https://doc.rust-lang.org/rust-by-example/error/result/enter_question_mark.html
fn split_int_pair(in_string: &str) -> Result<(i32,i32), ParseIntError> {
    let mut splitter = in_string.splitn(2, "   ");
    let first = splitter.next().unwrap().parse::<i32>()?;
    let second = splitter.next().unwrap().parse::<i32>()?;
    Ok((first, second))
}

fn main() -> std::io::Result<()>{
    let path = env::current_dir()?;
    println!("Hello, world! from dir {}", path.display());

    let contents = fs::read_to_string("data/input")
        .expect("Should have been able to read the file");

    // Split into lines
    let lines = contents.split("\n");

    let mut left_column_values:Vec<i32>=Vec::new();
    let mut right_column_values:Vec<i32>=Vec::new();

    for line in lines
    {
        match split_int_pair(line) {
            Ok((left_column_value,right_column_value))  =>
                {
                    //println!("values are is {left_column_value},{right_column_value}");
                    left_column_values.push(left_column_value);
                    right_column_values.push(right_column_value);
                },
            Err(e) => println!("Error: {}", e),
        }
    }
    assert_eq!(left_column_values.iter().count(), right_column_values.iter().count());

    left_column_values.sort();
    right_column_values.sort();

    let left_iter = left_column_values.iter();
    let mut right_iter = right_column_values.iter();
    let mut total_distance=0;
    for left_val in left_iter {
        let right_val=right_iter.next().unwrap();
        let distance=(right_val-left_val).abs();
        total_distance+=distance;
        println!("Sorted Values {left_val},{right_val} distance {distance}")
    }

    println!("Total distance {total_distance}");

    Ok(())
}

