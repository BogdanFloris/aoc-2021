struct Card {
    card: Vec<Vec<u32>>,
    bingo: bool,
}

impl Card {
    fn check_number(&mut self, number: u32) {
        for row in self.card.iter_mut() {
            for cell in row.iter_mut() {
                if *cell == number {
                    *cell = 100;
                }
            }
        }
    }

    fn check_bingo(&mut self) -> bool {
        let mut bingo = false;
        // Check rows
        for row in self.card.iter() {
            if row.iter().all(|&x| x == 100) {
                bingo = true;
            }
        }
        // Check columns
        for i in 0..5 {
            if self.card.iter().all(|row| row[i] == 100) {
                bingo = true;
            }
        }
        self.bingo = bingo;
        bingo
    }

    fn replace_all_100_with_0(&mut self) {
        for row in self.card.iter_mut() {
            for cell in row.iter_mut() {
                if *cell == 100 {
                    *cell = 0;
                }
            }
        }
    }

    fn sum(&self) -> u32 {
        self.card.iter().map(|row| row.iter().sum::<u32>()).sum()
    }
}

struct Bingo {
    cards: Vec<Card>,
    numbers: Vec<u32>,
    results: Vec<u32>,
}

impl Bingo {
    fn from_str(input: &str) -> Bingo {
        let lines: Vec<&str> = input.trim().split("\n\n").collect();
        let numbers = lines[0].split(',').map(|x| x.parse().unwrap()).collect();
        let cards = lines[1..]
            .iter()
            .map(|x| {
                let card = x
                    .split('\n')
                    .map(|y| {
                        y.split_whitespace()
                            .map(|z| z.trim().parse().unwrap())
                            .collect()
                    })
                    .collect();
                Card { card, bingo: false }
            })
            .collect();
        Bingo {
            cards,
            numbers,
            results: Vec::new(),
        }
    }

    fn play(&mut self) {
        for number in self.numbers.iter() {
            for card in self.cards.iter_mut() {
                if card.bingo {
                    continue;
                }
                card.check_number(*number);
                if card.check_bingo() {
                    card.replace_all_100_with_0();
                    self.results.push(*number * card.sum());
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut bingo = Bingo::from_str(input);
    bingo.play();
    Some(bingo.results[0])
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut bingo = Bingo::from_str(input);
    bingo.play();
    Some(bingo.results[bingo.results.len() - 1])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(4512));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(1924));
    }
}
