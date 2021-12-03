use std::collections::VecDeque;
use std::num::ParseIntError;

use crate::common;

pub fn run<T>(depth_lines: T, window_size: usize) -> common::Result<()>
where
    T: Iterator<Item = Result<i32, ParseIntError>>,
{
    let num_increases = count_increases_windowed(depth_lines, window_size)?;
    println!("The depth increased {} times.", num_increases);
    Ok(())
}

fn count_increases_windowed<T>(depths: T, window_size: usize) -> common::Result<u32>
where
    T: Iterator<Item = Result<i32, ParseIntError>>,
{
    let mut num_increases = 0;
    let mut window: VecDeque<i32> = VecDeque::new();
    for depth in depths {
        let safe_depth: i32 = depth?;
        if window.len() >= window_size {
            let previous_start: i32 = window.pop_front().unwrap();
            let window_rest_sum: i32 = window.iter().sum();
            let previous_sum = previous_start + window_rest_sum;
            let next_sum = safe_depth + window_rest_sum;
            if previous_sum < next_sum {
                num_increases += 1;
            }
        }
        window.push_back(safe_depth)
    }
    Ok(num_increases)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_increases_gets_example_correct() {
        let test_str = String::from("199\n200\n208\n210\n200\n207\n240\n269\n260\n263");
        let num_increases = count_increases_windowed(common::int_lines(&test_str), 1);
        assert!(num_increases.is_ok());
        assert_eq!(7, num_increases.unwrap());
    }

    #[test]
    fn windowed_count_increases_gets_example_correct() {
        let test_str = String::from("199\n200\n208\n210\n200\n207\n240\n269\n260\n263");
        let num_increases = count_increases_windowed(common::int_lines(&test_str), 3);
        assert!(num_increases.is_ok());
        assert_eq!(5, num_increases.unwrap())
    }
}
