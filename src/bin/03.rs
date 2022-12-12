struct BinaryDiagnostic {
    // binary diagnostic code
    numbers: Vec<String>,
}

impl BinaryDiagnostic {
    fn from_str(input: &str) -> BinaryDiagnostic {
        let numbers = input
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        BinaryDiagnostic { numbers }
    }

    fn get_gamma_and_epsilon_rates(&self) -> (String, String) {
        let mut gamma_rate = "".to_string();
        let mut epsilon_rate = "".to_string();
        let binary_length = self.numbers[0].len();
        for i in 0..binary_length {
            // get all bits at current position
            let bits: Vec<char> = self
                .numbers
                .iter()
                .map(|s| s.chars().nth(i).unwrap())
                .collect();
            // count 1s and 0s
            let ones = bits.iter().filter(|&b| *b == '1').count();
            if ones > self.numbers.len() / 2 {
                gamma_rate.push('1');
                epsilon_rate.push('0');
            } else {
                gamma_rate.push('0');
                epsilon_rate.push('1');
            }
        }
        (gamma_rate, epsilon_rate)
    }

    fn get_oxygen_generator_rating(filtered_numbers: Vec<String>, index: usize) -> String {
        if filtered_numbers.len() == 1 {
            return filtered_numbers[0].clone();
        }
        let bits: Vec<char> = filtered_numbers
            .iter()
            .map(|s| s.chars().nth(index).unwrap())
            .collect();
        let ones = bits.iter().filter(|&b| *b == '1').count();
        let new_filtered_numbers: Vec<String> = if ones >= filtered_numbers.len() - ones {
            // filter items with 1 at current position
            filtered_numbers
                .iter()
                .filter(|s| s.chars().nth(index).unwrap() == '1')
                .map(|s| s.to_string())
                .collect()
        } else {
            // filter items with 0 at current position
            filtered_numbers
                .iter()
                .filter(|s| s.chars().nth(index).unwrap() == '0')
                .map(|s| s.to_string())
                .collect()
        };
        BinaryDiagnostic::get_oxygen_generator_rating(new_filtered_numbers, index + 1)
    }

    fn get_co2_scrubber_rating(filtered_numbers: Vec<String>, index: usize) -> String {
        if filtered_numbers.len() == 1 {
            return filtered_numbers[0].clone();
        }
        let bits: Vec<char> = filtered_numbers
            .iter()
            .map(|s| s.chars().nth(index).unwrap())
            .collect();
        let zeroes = bits.iter().filter(|&b| *b == '0').count();
        let new_filtered_numbers: Vec<String> = if zeroes <= filtered_numbers.len() - zeroes {
            // filter items with 0 at current position
            filtered_numbers
                .iter()
                .filter(|s| s.chars().nth(index).unwrap() == '0')
                .map(|s| s.to_string())
                .collect()
        } else {
            // filter items with 1 at current position
            filtered_numbers
                .iter()
                .filter(|s| s.chars().nth(index).unwrap() == '1')
                .map(|s| s.to_string())
                .collect()
        };
        BinaryDiagnostic::get_co2_scrubber_rating(new_filtered_numbers, index + 1)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let binary_diagnostic = BinaryDiagnostic::from_str(input);
    let (gamma_rate_str, epsilon_rate_str) = binary_diagnostic.get_gamma_and_epsilon_rates();
    let gamma_rate = u32::from_str_radix(&gamma_rate_str, 2).unwrap();
    let epsilon_rate = u32::from_str_radix(&epsilon_rate_str, 2).unwrap();
    Some(gamma_rate * epsilon_rate)
}

pub fn part_two(input: &str) -> Option<u32> {
    let binary_diagnostic = BinaryDiagnostic::from_str(input);
    let oxygen_generator_rating_str =
        BinaryDiagnostic::get_oxygen_generator_rating(binary_diagnostic.numbers.clone(), 0);
    let co2_scrubber_rating_str =
        BinaryDiagnostic::get_co2_scrubber_rating(binary_diagnostic.numbers, 0);
    let oxygen_generator_rating = u32::from_str_radix(&oxygen_generator_rating_str, 2).unwrap();
    let co2_scrubber_rating = u32::from_str_radix(&co2_scrubber_rating_str, 2).unwrap();
    Some(oxygen_generator_rating * co2_scrubber_rating)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(198));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(230));
    }
}
