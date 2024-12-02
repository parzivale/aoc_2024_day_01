use std::env;
use std::fs;

fn split_int_pair(in_string: &str) -> (i32, i32) {
    let mut splitter = in_string.splitn(2, "   ");
    let first = splitter.next().unwrap().parse::<i32>().unwrap();
    let second = splitter.next().unwrap().parse::<i32>().unwrap();
    (first, second)
}

fn main() -> std::io::Result<()>{
    let path = env::current_dir()?;
    println!("Hello, world! from dir {}", path.display());

    let contents = fs::read_to_string("data/input")
        .expect("Should have been able to read the file");
    
    // Split into lines
    let lines = contents.split("\n");
    //let line_count =lines.count();
    //println!("With text:\n{line_count}");
    for line in lines
    {
        println!("{line}");
        let (left_column_value, right_column_value) = split_int_pair(line);
        println!("{left_column_value},{right_column_value}");
    }
    Ok(())
}
