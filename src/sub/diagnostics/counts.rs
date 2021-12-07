use crate::sub::diagnostics::common::int_from_bit_iter;

pub struct DigitCounts {
    counts: Vec<[usize; 2]>,
}

impl DigitCounts {
    pub fn new() -> DigitCounts {
        DigitCounts { counts: vec![] }
    }

    pub fn from_binary_numbers(numbers: &Vec<Vec<u8>>) -> DigitCounts {
        let mut digit_counts = DigitCounts::new();
        let first_number = numbers.get(0);
        match first_number {
            Some(first_vec) => {
                digit_counts.init_to_size(first_vec.len());
                for number_vec in numbers {
                    digit_counts.add(number_vec);
                }
                digit_counts
            }
            None => digit_counts,
        }
    }

    pub fn init_to_size(&mut self, size: usize) {
        for _ in 0..size {
            self.counts.push([0; 2]);
        }
    }

    pub fn add(&mut self, binary_number: &Vec<u8>) {
        for (i, digit) in binary_number.iter().enumerate() {
            self.counts[i][*digit as usize] += 1;
        }
    }

    pub fn len(&self) -> usize {
        self.counts.len()
    }

    pub fn get_int_value_from_comparison(&self, cmp: fn(usize, usize) -> bool) -> u32 {
        let bit_vec: Vec<u32> = self
            .counts
            .iter()
            .map(|counts_for_index| choose_bit_from_cmp(counts_for_index, cmp) as u32)
            .collect();
        int_from_bit_iter(bit_vec.iter())
    }

    pub fn value_for(&self, cmp: fn(usize, usize) -> bool, index: usize) -> u8 {
        choose_bit_from_cmp(&self.counts[index], cmp)
    }
}

fn choose_bit_from_cmp(counts_for_index: &[usize; 2], cmp: fn(usize, usize) -> bool) -> u8 {
    if cmp(counts_for_index[0], counts_for_index[1]) {
        0
    } else {
        1
    }
}
