use std::collections::HashMap;
fn main() {
    let part_one_input = include_str!("../../inputs/day_1.txt");
    let part_one_result = part_one(part_one_input);

    println!("Part One: {part_one_result}");

    let part_two_result = part_two(part_one_input);

    println!("Part Two: {part_two_result}");
}

fn part_one(input: &str) -> i32 {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|l| {
            let nums: Vec<&str> = l.splitn(2, "   ").collect();
            let left = nums[0];
            let right = nums[1];
            let left = left.parse::<i32>().unwrap();
            let right = right.parse::<i32>().unwrap();

            (left, right)
        })
        .collect();

    left.sort();
    right.sort();

    left.into_iter().zip(right).fold(0, |acc, (l, r)| {
        let dist = i32::abs(l - r);

        acc + dist
    })
}

fn part_two(input: &str) -> i32 {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|l| {
            let nums: Vec<&str> = l.splitn(2, "   ").collect();
            let left = nums[0];
            let right = nums[1];
            let left = left.parse::<i32>().unwrap();
            let right = right.parse::<i32>().unwrap();

            (left, right)
        })
        .collect();

    left.sort();
    right.sort();

    let mut frequency: HashMap<i32, i32> = HashMap::new();

    right.iter().for_each(|n| {
        //frequency[*n as usize] += 1
        let num = frequency.entry(*n).or_insert(0);
        *num += 1;
    });

    left.iter().fold(0, |acc, v| {
        let freq = frequency.get(v).unwrap_or(&0);
        acc + (*v * freq)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let input = r"3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(part_one(input), 11);
    }

    #[test]
    fn part_two_works() {
        let input = r"3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(part_two(input), 31);
    }
}
