use anyhow::Result;
use shared_functions::read_one_per_line;

// this function is very slow and definitely optimizable
// originally it was only used for task2
// I changed it afterwards to also support task 1, which was solved a bit differently before
fn sum_symmetrical_numbers(f: &dyn Fn(&u64) -> usize) -> u64 {
    read_one_per_line::<String>("./data/input2.txt")
        .unwrap()
        .get(0)
        .unwrap()
        .split(",")
        .fold(Vec::<u64>::new(), |numbers, interval| {
            if let Some((start, end)) = interval.split_once("-") {
                (start.parse::<u64>().unwrap()..end.parse::<u64>().unwrap())
                    .chain(numbers)
                    .collect()
            } else {
                numbers
            }
        })
        .into_iter()
        .filter(|number| {
            if number.to_string().len() == 1 {
                false
            } else {
                (f(number)..number.to_string().len() / 2 + 1)
                    .map(|window_size| {
                        number
                            .to_string()
                            .chars()
                            .collect::<Vec<char>>()
                            .windows(window_size)
                            .step_by(window_size)
                            .collect::<Vec<&[char]>>()
                            .windows(2)
                            .all(|windows| {
                                windows[0] == windows[1]
                                    && number.to_string().len() % window_size == 0
                            })
                    })
                    .any(|num| num)
            }
        })
        .collect::<Vec<u64>>()
        .into_iter()
        .sum()
}

fn main() -> Result<()> {
    println!(
        "Task 1: {}",
        sum_symmetrical_numbers(&|x| { (x.to_string().len() + 1) / 2 })
    );
    println!("Task 2: {}", sum_symmetrical_numbers(&|_| { 1 }));

    Ok(())
}
