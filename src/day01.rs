use core::panic;
use std::str::FromStr;

use anyhow::Result;
use shared_functions::read_one_per_line;

#[derive(Debug)]
enum Rotation {
    Right(i32),
    Left(i32),
}

impl FromStr for Rotation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((direction, num)) = s.split_at_checked(1) {
            let steps = num.parse::<i32>()?;

            Ok(match direction {
                "R" => Rotation::Right(steps),
                "L" => Rotation::Left(steps),
                _ => panic!("Unhandled direction."),
            })
        } else {
            Err(anyhow::format_err!("Could not parse list."))
        }
    }
}

fn count_zeros() -> Result<usize> {
    Ok(read_one_per_line::<Rotation>("./data/input1.txt")?
        .iter()
        .scan(50, |loc, rot| {
            if let Some(new_loc) = match *rot {
                Rotation::Right(steps) => Some((*loc + steps).rem_euclid(100)),
                Rotation::Left(steps) => Some((*loc - steps).rem_euclid(100)),
            } {
                *loc = new_loc;
                Some(new_loc)
            } else {
                None
            }
        })
        .filter(|&p| p == 0)
        .count())
}

// this is not that nice and should be quite improvable
fn count_zeros_2() -> Result<i32> {
    Ok(read_one_per_line::<Rotation>("./data/input1.txt")?
        .iter()
        .fold((50, 0), |(loc, sum_zeroes), rot| {
            if let Some((new_loc, num_zeroes)) = match *rot {
                Rotation::Right(steps) => {
                    let result = (loc + steps).rem_euclid(100);
                    let zeroes = (loc + steps) / 100;
                    Some((result, zeroes))
                }
                Rotation::Left(steps) => {
                    let result = (loc - steps).rem_euclid(100);
                    let zeroes = if loc == 0 {
                        steps / 100
                    } else if loc - steps <= 0 {
                        ((steps - loc) / 100) + 1
                    } else {
                        0
                    };
                    Some((result, zeroes))
                }
            } {
                let new_sum_zeroes = sum_zeroes + num_zeroes;
                (new_loc, new_sum_zeroes)
            } else {
                println!("NOT GOOD.");
                (0, 0)
            }
        })
        .1)
}

fn main() -> Result<()> {
    println!("Part 1: {}", count_zeros()?);
    println!("Part 2: {}", count_zeros_2()?);

    Ok(())
}
