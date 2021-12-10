mod seven_segment;

use crate::Result;
pub use seven_segment::*;

pub fn filter_non_unique_digits(outputs: &Vec<OutputDisplay>) -> Result<Vec<&u8>> {
    let unique_digits: Vec<&u8> = outputs
        .iter()
        .map(|output| output.unique_digits())
        .flatten()
        .collect();
    Ok(unique_digits)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STR: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\nedbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\nfgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\nfbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\naecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\nfgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\ndbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\nbdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\negadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\ngcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn deduce_scrambled_works_for_test_data() -> Result<()> {
        let outputs = get_test_outputs()?;
        let unique_digits = filter_non_unique_digits(&outputs)?;
        assert_eq!(26, unique_digits.len());
        Ok(())
    }

    fn get_test_outputs() -> Result<Vec<OutputDisplay>> {
        let output_strings: Vec<&str> = TEST_STR
            .lines()
            .map(|line| line.split(" | ").last())
            .filter_map(|output_str| output_str)
            .collect();
        output_strings
            .iter()
            .map(|&output_str| OutputDisplay::from_str(output_str))
            .collect::<Result<Vec<OutputDisplay>>>()
    }
}
