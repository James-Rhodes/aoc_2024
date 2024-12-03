fn main() {
    let input = include_str!("../../inputs/day_2.txt");
    let part_one_result = part_one(input);
    println!("Part One Result: {part_one_result}");

    let part_two_result = part_two(input);
    println!("Part Two Result: {part_two_result}");
}

fn part_one(input: &str) -> usize {
    let reports: Vec<Report> = input
        .lines()
        .map(|l| {
            let levels = l.split_whitespace().map(|v| v.parse().unwrap()).collect();
            Report { levels }
        })
        .collect();

    reports.iter().filter(|r| r.is_safe_part_one()).count()
}

fn part_two(input: &str) -> usize {
    let reports: Vec<Report> = input
        .lines()
        .map(|l| {
            let levels = l.split_whitespace().map(|v| v.parse().unwrap()).collect();
            Report { levels }
        })
        .collect();
    reports.iter().filter(|r| r.is_safe_part_two()).count()
}

#[derive(Debug)]
struct Report {
    levels: Vec<usize>,
}

impl Report {
    fn is_safe_part_one(&self) -> bool {
        if (self.all_increasing().is_none() || self.all_decreasing().is_none())
            && self.adjacent_differences_safe().is_none()
        {
            return true;
        }
        false
    }

    fn is_safe_part_two(&self) -> bool {
        // My motto for this one is that if it is dumb but works, then it isn't dumb
        if self.is_safe_part_one() {
            return true;
        }

        for i in 0..self.levels.len() {
            let mut new_levels = self.levels.clone();
            new_levels.remove(i);
            let new_report = Report { levels: new_levels };
            if new_report.is_safe_part_one() {
                return true;
            }
        }
        false
    }

    fn all_increasing(&self) -> Option<usize> {
        for (i, win) in self.levels.windows(2).enumerate() {
            let a = win[0];
            let b = win[1];
            if a >= b {
                return Some(i);
            }
        }

        return None;
    }

    fn all_decreasing(&self) -> Option<usize> {
        for (i, win) in self.levels.windows(2).enumerate() {
            let a = win[0];
            let b = win[1];
            if a <= b {
                return Some(i);
            }
        }

        return None;
    }

    fn adjacent_differences_safe(&self) -> Option<usize> {
        for (i, win) in self.levels.windows(2).enumerate() {
            let a = win[0];
            let b = win[1];

            let diff = usize::abs_diff(a, b);
            if diff < 1 || diff > 3 {
                return Some(i);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let input = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        assert_eq!(part_one(input), 2);
    }
    #[test]
    fn part_two_works() {
        let input = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        assert_eq!(part_two(input), 4);
    }
}