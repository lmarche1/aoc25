use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn main() {
    let input: Vec<String> = read_lines("./data/test1.txt");

    let tuples = input
        .iter()
        .map(|i| i.split_at(1))
        .collect::<Vec<(&str, &str)>>();

    for i in tuples {
        println!("Direction: {}, Number: {}", i.0, i.1);
    }
}
