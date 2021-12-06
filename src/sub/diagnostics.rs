mod common;
mod counts;
mod filters;
mod frequency;

use crate::Result;

pub fn run_gamma_epsilon_report(binary_numbers: Vec<Vec<u8>>) -> Result<()> {
    let digit_counts = counts::DigitCounts::from_binary_numbers(&binary_numbers);
    let gamma_rate = frequency::get_gamma_rate_from_counts(&digit_counts);
    let epsilon_rate = frequency::get_epsilon_rate_from_counts(&digit_counts);
    println!("Power consumption: {}", gamma_rate * epsilon_rate);
    Ok(())
}

pub fn run_life_support_rating_report(binary_numbers: Vec<Vec<u8>>) -> Result<()> {
    let oxygen_rating = filters::find_oxygen_generator_rating(&binary_numbers)?;
    let co2_rating = filters::find_co2_scrubber_rating(&binary_numbers)?;
    println!("Life support rating: {}", oxygen_rating * co2_rating);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::bit_lines;

    const TEST_STR: &str =
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

    #[test]
    fn gets_correct_gamma_and_epsilon_for_test_data() {
        let counts = counts::DigitCounts::from_binary_numbers(&bit_lines(TEST_STR));
        assert_eq!(22, frequency::get_gamma_rate_from_counts(&counts));
        assert_eq!(9, frequency::get_epsilon_rate_from_counts(&counts));
    }
}
