use super::counts::DigitCounts;
use super::frequency;
use crate::sub::diagnostics::common::int_from_bit_iter;
use crate::{Error, Result};

pub fn find_oxygen_generator_rating(binary_numbers: &Vec<Vec<u8>>) -> Result<u32> {
    find_system_rating("oxygen generator", binary_numbers, frequency::most_common)
}

pub fn find_co2_scrubber_rating(binary_numbers: &Vec<Vec<u8>>) -> Result<u32> {
    find_system_rating("CO2 scrubber", binary_numbers, frequency::least_common)
}

fn find_system_rating(
    system_label: &str,
    binary_numbers: &Vec<Vec<u8>>,
    cmp: fn(usize, usize) -> bool,
) -> Result<u32> {
    let first_element = binary_numbers.first();
    match first_element {
        Some(_) => match find_bit_vector_using_cmp(binary_numbers, 0, cmp) {
            Some(co2_rating_vec) => Ok(int_from_bit_iter(co2_rating_vec.iter())),
            None => Err(Box::new(Error::new(&format!(
                "Couldn't find {} rating in vector!",
                system_label
            )))),
        },
        None => Err(Box::new(Error::new(&format!(
            "Empty vector provided to find {} rating!",
            system_label
        )))),
    }
}

fn find_bit_vector_using_cmp(
    binary_numbers: &Vec<Vec<u8>>,
    current_digit: usize,
    cmp: fn(usize, usize) -> bool,
) -> Option<Vec<u8>> {
    let counts = DigitCounts::from_binary_numbers(binary_numbers);
    let first_element = binary_numbers.first();
    if binary_numbers.len() <= 1 {
        return first_element.map(|vec_ref| vec_ref.clone());
    }
    let filtered_binary_numbers: Vec<Vec<u8>> = binary_numbers
        .iter()
        .filter_map(|number_vec| {
            let actual_digit = number_vec[current_digit];
            let most_common = counts.value_for(cmp, current_digit);
            if actual_digit == most_common {
                Some(number_vec.clone())
            } else {
                None
            }
        })
        .collect();
    if current_digit >= counts.len() {
        filtered_binary_numbers
            .first()
            .map(|vec_ref| vec_ref.clone())
    } else {
        find_bit_vector_using_cmp(&filtered_binary_numbers, current_digit + 1, cmp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::bit_lines;

    const TEST_STR: &str =
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

    #[test]
    fn can_find_oxygen_rating() {
        let result = find_oxygen_generator_rating(&bit_lines(TEST_STR));
        assert!(result.is_ok());
        assert_eq!(23, result.unwrap());
    }
    #[test]
    fn can_find_co2_rating() {
        let result = find_co2_scrubber_rating(&bit_lines(TEST_STR));
        assert!(result.is_ok());
        assert_eq!(10, result.unwrap());
    }
}
