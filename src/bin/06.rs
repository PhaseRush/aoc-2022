use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    for i in 4..input.len() {
        let substr = &input[i - 4..i];
        let set: HashSet<char> = substr.chars().into_iter().collect();
        if set.len() == 4 {
            return Some((i) as u32);
        }
    }
    return None;
}

pub fn part_two(input: &str) -> Option<u32> {
    for i in 14..input.len() {
        let substr = &input[i - 14..i];
        let set: HashSet<char> = substr.chars().into_iter().collect();
        if set.len() == 14 {
            return Some((i) as u32);
        }
    }
    return None;
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
