use std::str::FromStr;

use anyhow::Result;
use shared_functions::read_one_per_line;

#[derive(Debug)]
struct Battery {
    line: String,
    joltage: u32,
}

impl FromStr for Battery {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Some((num1, num2)) =
            s.chars()
                .rfold(Some(('0', '0')), |n: Option<(char, char)>, c| match n {
                    Some(nums) => {
                        if c >= nums.0 || nums.1 == '0' {
                            Some((c, char::max(nums.0, nums.1)))
                        } else {
                            Some(nums)
                        }
                    }
                    None => None,
                })
        {
            Ok(Battery {
                line: s.to_string(),
                joltage: num1.to_digit(10).unwrap() * 10 + num2.to_digit(10).unwrap(),
            })
        } else {
            Err(anyhow::format_err!("Line mismatch."))
        }
    }
}

fn main() -> Result<()> {
    let battery_lines = read_one_per_line::<Battery>("./data/input3.txt")?;

    println!(
        "Task 1: {}",
        battery_lines
            .iter()
            .fold(0, |s: u32, b: &Battery| { b.joltage + s })
    );

    Ok(())
}
