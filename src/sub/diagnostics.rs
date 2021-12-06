use crate::Result;

pub fn run_gamma_epsilon_report(binary_numbers: Vec<Vec<u8>>) -> Result<()> {
    let counts_vec = count_digits_in_binary_numbers(binary_numbers);
    let gamma_rate = get_gamma_rate_from_counts(&counts_vec);
    let epsilon_rate = get_epsilon_rate_from_counts(&counts_vec);
    println!("Power consumption: {}", gamma_rate * epsilon_rate);
    Ok(())
}

fn count_digits_in_binary_numbers(numbers: Vec<Vec<u8>>) -> Vec<[usize; 2]> {
    let mut counts_vec: Vec<[usize; 2]> = Vec::new();
    let first_number = numbers.get(0);
    match first_number {
        Some(first_vec) => {
            for _ in 0..first_vec.len() {
                counts_vec.push([0; 2]);
            }
            for number_vec in numbers {
                for (i, digit) in number_vec.iter().enumerate() {
                    counts_vec[i][*digit as usize] += 1;
                }
            }
            counts_vec
        }
        None => counts_vec,
    }
}

fn get_gamma_rate_from_counts(counts: &Vec<[usize; 2]>) -> u32 {
    get_int_value_based_on_frequency(counts, |zeroes, ones| zeroes >= ones)
}

fn get_epsilon_rate_from_counts(counts: &Vec<[usize; 2]>) -> u32 {
    get_int_value_based_on_frequency(counts, |zeroes, ones| zeroes <= ones)
}

fn get_int_value_based_on_frequency(
    counts: &Vec<[usize; 2]>,
    frequency_cmp: fn(usize, usize) -> bool,
) -> u32 {
    bit_iter_to_int(
        counts
            .iter()
            .map(|counts_for_index| get_bit_for_frequency(counts_for_index, frequency_cmp)),
    )
}

fn get_bit_for_frequency(counts_array: &[usize; 2], cmp: fn(usize, usize) -> bool) -> u32 {
    if cmp(counts_array[0], counts_array[1]) {
        0
    } else {
        1
    }
}

fn bit_iter_to_int<T>(bit_iter: T) -> u32
where
    T: ExactSizeIterator + DoubleEndedIterator<Item = u32>,
{
    bit_iter
        .rev()
        .enumerate()
        .rfold(0, |acc, (i, c)| acc + (c << i))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::bit_lines;

    const TEST_STR: &str =
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

    #[test]
    fn gets_correct_gamma_and_epsilon_for_test_data() {
        let binary_numbers = bit_lines(TEST_STR);
        let counts_vec = count_digits_in_binary_numbers(binary_numbers);
        assert_eq!(22, get_gamma_rate_from_counts(&counts_vec));
        assert_eq!(9, get_epsilon_rate_from_counts(&counts_vec));
    }
}
