use crate::Errors::RoundParseError;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Throw {
    Rock,
    Paper,
    Scissors,
}

fn get_points(throw: &Throw) -> u32 {
    match throw {
        Throw::Rock => 1,
        Throw::Paper => 2,
        Throw::Scissors => 3,
    }
}

enum Errors {
    ThrowParseError,
    RoundParseError,
}

impl TryFrom<char> for Throw {
    type Error = Errors;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Throw::Rock),
            'B' | 'Y' => Ok(Throw::Paper),
            'C' | 'Z' => Ok(Throw::Scissors),
            _ => Err(Self::Error::ThrowParseError),
        }
    }
}

struct Round {
    player: Throw,
    opponent: Throw,
}

struct Round2 {
    player: Throw,
    opponent: Throw,
}

impl Round {
    fn get_points(&self) -> u32 {
        get_points(&self.player) + self.winner_points()
    }

    fn winner_points(&self) -> u32 {
        match (&self.player, &self.opponent) {
            (Throw::Rock, Throw::Rock) | (Throw::Paper, Throw::Paper) | (Throw::Scissors, Throw::Scissors) => 3,
            (Throw::Rock, Throw::Scissors)
            | (Throw::Scissors, Throw::Paper)
            | (Throw::Paper, Throw::Rock) => 6,
            _ => 0,
        }
    }
}

impl TryFrom<&str> for Round {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.as_bytes().len() != 3 {
            return Err(Self::Error::RoundParseError);
        }

        let opponent = value.as_bytes()[0];
        let player = value.as_bytes()[2];

        Ok(Round {
            player: Throw::try_from(char::from(player))?,
            opponent: Throw::try_from(char::from(opponent))?,
        })
    }
}


impl Round2 {
    fn get_points(&self) -> u32 {
        get_points(&self.player) + self.winner_points()
    }

    fn winner_points(&self) -> u32 {
        match (&self.player, &self.opponent) {
            (Throw::Rock, Throw::Rock) | (Throw::Paper, Throw::Paper) | (Throw::Scissors, Throw::Scissors) => 3,
            (Throw::Rock, Throw::Scissors)
            | (Throw::Scissors, Throw::Paper)
            | (Throw::Paper, Throw::Rock) => 6,
            _ => 0,
        }
    }
}


impl TryFrom<&str> for Round2 {
    type Error = crate::Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.as_bytes().len() != 3 {
            return Err(Self::Error::RoundParseError);
        }

        let opponent = value.as_bytes()[0];
        let player = char::try_from(value.as_bytes()[2] as u32).expect("guaranteed to be safe");

        let opponent = Throw::try_from(char::from(opponent));
        let opponent = match opponent {
            Ok(v) => {v}
            Err(_) => {return Err(RoundParseError)}
        };

        let player = match player {
            'X' => {
                match &opponent {
                    Throw::Rock => { Throw::Scissors }
                    Throw::Scissors => { Throw::Paper }
                    Throw::Paper => { Throw::Rock }
                }
            }
            'Y' => opponent,
            'Z' => {
                match &opponent {
                    Throw::Rock => { Throw::Paper }
                    Throw::Scissors => { Throw::Rock }
                    Throw::Paper => { Throw::Scissors }
                }
            }
            _ => {panic!("Must only be X, Y, or Z")}
        };

        Ok(Round2 {
            player,
            opponent,
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let x = input
        .lines()
        .filter_map(|val| Round::try_from(val).ok())
        .map(|rnd| rnd.get_points())
        .sum();

    Some(x)

    // println!("\n\n\ntest: {:?}",x );
}

pub fn part_two(input: &str) -> Option<u32> {
    let x = input
        .lines()
        .filter_map(|val| Round2::try_from(val).ok())
        .map(|rnd| rnd.get_points())
        .sum();

    Some(x)
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
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
