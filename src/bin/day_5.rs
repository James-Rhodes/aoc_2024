fn main() {
    let input = include_str!("../../inputs/day_5.txt");
    let part_one_result = part_one(input);

    println!("Part One: {part_one_result}");

    let part_two_result = part_two(input);

    println!("Part Two: {part_two_result}");
}

fn part_one(input: &str) -> usize {
    let (rules, updates) = parse_input(input);

    let mut result = 0;
    for update in updates {
        if update_is_valid(&update, &rules) {
            let middle_idx = update.len() / 2;
            result += update[middle_idx];
        }
    }

    result
}

fn part_two(input: &str) -> usize {
    let (rules, updates) = parse_input(input);

    let mut result = 0;
    for update in updates {
        if !update_is_valid(&update, &rules) {
            let fixed_update = fix_update(&update, &rules);
            let middle_idx = fixed_update.len() / 2;
            result += fixed_update[middle_idx];
        }
    }

    result
}

fn parse_input(input: &str) -> (Rules, Updates) {
    let (rule_definitions, updates) = input.split_once("\n\n").unwrap();

    let mut rules = vec![vec![]; 100]; // 99 is the max number so just use a vector as a look up table. (I think even 10 is the min number but just to be safe)

    rule_definitions.lines().for_each(|l| {
        let (before, after) = l.split_once("|").unwrap();
        let before = before.parse::<usize>().unwrap();
        let after = after.parse::<usize>().unwrap();

        rules[before].push(after);
    });

    // Sort so I can use binary search later.
    // NOTE: This ended up being a premature optimization. The list of rules is only small so a
    // linear search was just as fast. Leaving it in, but this note is a slap on the wrist.
    rules.iter_mut().for_each(|rule| rule.sort_unstable());

    let updates: Updates = updates
        .lines()
        .map(|l| l.split(",").map(|v| v.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

fn update_is_valid(update: &Update, rules: &Rules) -> bool {
    for i in 0..(update.len() - 1) {
        for j in (i + 1)..update.len() {
            //Bubble Sort Kind of Procedure
            if !a_proceeds_b_correctly(update[i], update[j], rules) {
                return false;
            }
        }
    }

    true
}

fn fix_update(update: &Update, rules: &Rules) -> Update {
    // Maybe need to not clone, see how slow it is
    let mut update = update.clone();

    for i in 0..(update.len() - 1) {
        for j in (i + 1)..update.len() {
            //Bubble Sort Kind of Procedure
            if !a_proceeds_b_correctly(update[i], update[j], rules) {
                let temp = update[i];
                update[i] = update[j];
                update[j] = temp;
            }
        }
    }
    update
}

type Rules = Vec<Vec<usize>>;
type Update = Vec<usize>;
type Updates = Vec<Update>;

fn a_proceeds_b_correctly(a: usize, b: usize, rules: &Rules) -> bool {
    let b_rules = &rules[b];
    if b_rules.binary_search(&a).is_ok() {
        // Found a rule saying b should be before a. So this is not correct.
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let input = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        assert_eq!(part_one(input), 143);
    }

    #[test]
    fn part_two_works() {
        let input = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        assert_eq!(part_two(input), 123);
    }
}
