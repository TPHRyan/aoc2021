pub mod seven_segment;
mod unscramble;

use seven_segment::*;
pub use unscramble::unscramble_outputs;

pub fn filter_non_unique_digits(outputs: &Vec<OutputDisplay>) -> Vec<&u8> {
    let unique_digits: Vec<&u8> = outputs
        .iter()
        .map(|output| output.unique_digits())
        .flatten()
        .collect();
    unique_digits
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    const TEST_STR: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\nedbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\nfgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\nfbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\naecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\nfgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\ndbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\nbdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\negadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\ngcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn deduce_scrambled_works_for_test_data() -> Result<()> {
        let outputs = get_test_outputs()?;
        let unique_digits = filter_non_unique_digits(&outputs);
        assert_eq!(26, unique_digits.len());
        Ok(())
    }

    #[test]
    fn unscramble_outputs_works_for_test_data() -> Result<()> {
        let test_data = get_test_data()?;
        let unscrambled_outputs = unscramble_outputs(&test_data)?;
        let output_sum: u32 = unscrambled_outputs
            .iter()
            .fold(0, |acc, &output_number| acc + output_number);
        assert_eq!(61229, output_sum);
        Ok(())
    }

    fn get_test_data() -> Result<Vec<(Vec<u8>, OutputDisplay)>> {
        signal_output_lines_to_signals_and_output(TEST_STR)
    }

    fn get_test_outputs() -> Result<Vec<OutputDisplay>> {
        let test_data = get_test_data()?;
        Ok(test_data.into_iter().map(|(_, output)| output).collect())
    }
}
