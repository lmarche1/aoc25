use anyhow::Result;
use shared_functions::read_one_per_line;

fn main() -> Result<()> {
    let task1: u64 = read_one_per_line::<String>("./data/input2.txt")?
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
            if let Some((num1, num2)) = number
                .to_string()
                .split_at_checked(number.to_string().len() / 2)
            {
                num1 == num2
            } else {
                false
            }
        })
        .collect::<Vec<u64>>()
        .into_iter()
        .sum();

    println!("{task1:?}");
    Ok(())
}
