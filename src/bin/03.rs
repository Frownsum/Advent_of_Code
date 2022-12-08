
fn get_priority(c: &char) -> u8{

    if c.is_lowercase() {
        *c as u8 - b'a'+ 1
    } else {
        *c as u8 - b'A'+ 27
    }

}

pub fn part_one(input: &str) -> Option<u32> {

    use std::collections::BTreeSet;

    let mut sum : u32 = 0;

    for line in input.lines(){

        let mut front: BTreeSet<char> = BTreeSet::new();
        let mut back: BTreeSet<char> = BTreeSet::new();

        let length = line.len();

        for (i, c) in line.chars().enumerate() {
            {
                let collection = if i < length/2 {
                    &mut front
                } else {
                    &mut back
                };

                collection.insert(c);
            }
        }

        let intersect : Vec<char> = front.intersection(&back).cloned().collect();

        match intersect.first() {
            Some(x) => {sum += get_priority(x) as u32;},
            None => {break;},
        };

    };

    Some(sum)
}

use std::collections::BTreeSet;

fn set_from_line(line: &str) -> BTreeSet<char>{

    let mut set: BTreeSet<char> = BTreeSet::new();
    for c in line.chars(){
        set.insert(c);
    }
    set
}

pub fn part_two(input: &str) -> Option<u32> {
    use std::collections::BTreeSet;

    let mut lines = input.lines();
    let mut sum = 0;

    while let (Some(a), Some(b), Some(c)) = (lines.next(), lines.next(), lines.next()){

        let front = set_from_line(&a);
        let mid = set_from_line(&b);
        let back = set_from_line(&c);

        let mask: BTreeSet<char> = mid.intersection(&back).cloned().collect();

        let letter:Vec<char> = front.intersection(&mask).cloned().collect();

        match letter.first() {
            Some(x) => {sum += get_priority(x) as u32;},
            None => {break;},
        };

    }

    Some(sum)
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
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
