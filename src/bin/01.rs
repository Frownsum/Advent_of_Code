pub fn part_one(input: &str) -> Option<u32> {
    let mut max_val: u32 = 0;

    for values in input.split("\n\n") {
        let v = values
            .lines()
            .into_iter()
            .map(|val| val.parse::<u32>().unwrap())
            .sum();
        max_val = std::cmp::max(max_val, v);
    }

    Some(max_val)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut max_vals: [u32; 3] = [0, 0, 0];
    for values in input.split("\n\n") {
        let v = values
            .lines()
            .into_iter()
            .map(|val| val.parse::<u32>().unwrap())
            .sum::<u32>();
        for max_val in max_vals.iter_mut() {
            if *max_val < v {
                *max_val = v;
                break;
            }
        }
        max_vals.sort();
    }
    Some(max_vals.iter().sum())
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
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
