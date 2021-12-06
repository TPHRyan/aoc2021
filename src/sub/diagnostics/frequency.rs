use super::counts::DigitCounts;

pub fn least_common(zero_count: usize, one_count: usize) -> bool {
    return zero_count <= one_count;
}

pub fn most_common(zero_count: usize, one_count: usize) -> bool {
    return zero_count > one_count;
}

pub fn get_gamma_rate_from_counts(counts: &DigitCounts) -> u32 {
    counts.get_int_value_from_comparison(most_common)
}

pub fn get_epsilon_rate_from_counts(counts: &DigitCounts) -> u32 {
    counts.get_int_value_from_comparison(least_common)
}
