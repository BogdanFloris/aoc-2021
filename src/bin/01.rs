struct SonarSweep {
    numbers: Vec<u32>,
}

impl SonarSweep {
    fn from_string(input: &str) -> SonarSweep {
        let numbers: Vec<u32> = input
            .trim()
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
            .collect();
        SonarSweep { numbers }
    }

    fn get_number_of_depth_increases(&self) -> u32 {
        let mut increases = 0;
        for i in 1..self.numbers.len() {
            if self.numbers[i] > self.numbers[i - 1] {
                increases += 1;
            }
        }
        increases
    }

    fn get_number_of_depth_increases_sliding_window(&self) -> u32 {
        let mut increases = 0;
        let mut last_sum = u32::MAX;
        for i in 1..self.numbers.len() - 1 {
            let sum = self.numbers[i - 1] + self.numbers[i] + self.numbers[i + 1];
            if sum > last_sum {
                increases += 1;
            }
            last_sum = sum;
        }
        increases
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let sweep = SonarSweep::from_string(input);
    Some(sweep.get_number_of_depth_increases())
}

pub fn part_two(input: &str) -> Option<u32> {
    let sweep = SonarSweep::from_string(input);
    Some(sweep.get_number_of_depth_increases_sliding_window())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(5));
    }
}
