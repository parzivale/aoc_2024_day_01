use std::env;
use std::fs;
use std::num::ParseIntError;
use std::collections::HashMap;

// From https://doc.rust-lang.org/rust-by-example/error/result/enter_question_mark.html
fn split_int_pair(in_string: &str) -> Result<(i32,i32), ParseIntError> {
    let mut splitter = in_string.splitn(2, "   ");
    let first = splitter.next().unwrap().parse::<i32>()?;
    let second = splitter.next().unwrap().parse::<i32>()?;
    Ok((first, second))
}

fn build_integer_map(int_vector : &Vec<i32>) -> HashMap<i32,i32> {
    let mut value_count_map = HashMap::<i32,i32>::new();

    for int_value_ref in int_vector.iter() {
        let int_value:i32=int_value_ref.abs();
        if (value_count_map.contains_key(int_value_ref))
        {
            // Increment
            *value_count_map.get_mut(int_value_ref).unwrap()+=1;
        } else {
            value_count_map.insert(int_value,1);
        }
    }
    value_count_map
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

    // Calculate similarity
    let left_count_map=build_integer_map(&left_column_values);
    let right_count_map=build_integer_map(&right_column_values);

    let mut total_similarity=0;

    for left_val in left_column_values.iter() {
        if (right_count_map.contains_key(left_val))
        {
            let right_count=right_count_map.get(left_val).unwrap();
            // Multiply
            total_similarity+=left_val*right_count;
            println!("Number {left_val} found count {right_count}");
        }
        else {
            println!("Number {left_val} not found in map")
        }

    }

    println!("Total similarity {total_similarity}");

    Ok(())
}

