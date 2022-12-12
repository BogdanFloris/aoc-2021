#[derive(Debug)]
enum Command {
    Forward,
    Down,
    Up,
}

struct Dive {
    coords: (i32, i32),
    aim: i32,
    commands: Vec<(Command, i32)>,
}

impl Dive {
    fn from_string(input: &str) -> Dive {
        let mut commands = Vec::new();

        for line in input.trim().lines() {
            let mut parts = line.split_whitespace();
            let command = match parts.next().unwrap() {
                "forward" => Command::Forward,
                "down" => Command::Down,
                "up" => Command::Up,
                _ => panic!("Unknown command"),
            };
            let value = parts.next().unwrap().parse::<i32>().unwrap();
            commands.push((command, value));
        }

        Dive {
            coords: (0, 0),
            aim: 0,
            commands,
        }
    }

    fn run_commands(&mut self) {
        for (command, value) in &self.commands {
            match command {
                Command::Forward => {
                    self.coords.0 += value;
                }
                Command::Down => {
                    self.coords.1 += value;
                }
                Command::Up => {
                    self.coords.1 -= value;
                }
            }
        }
    }

    fn run_commands_with_aim(&mut self) {
        for (command, value) in &self.commands {
            match command {
                Command::Forward => {
                    self.coords.0 += value;
                    self.coords.1 += self.aim * value;
                }
                Command::Down => {
                    self.aim += value;
                }
                Command::Up => {
                    self.aim -= value;
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut dive = Dive::from_string(input);
    dive.run_commands();
    Some(dive.coords.0 * dive.coords.1)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut dive = Dive::from_string(input);
    dive.run_commands_with_aim();
    Some(dive.coords.0 * dive.coords.1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(150));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(900));
    }
}
