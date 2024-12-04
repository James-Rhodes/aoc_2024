fn main() {
    let input = include_str!("../../inputs/day_3.txt");
    let part_one_result = part_one(input);

    println!("Part One: {part_one_result}");

    let part_two_result = part_two(input);

    println!("Part Two: {part_two_result}");
}

fn part_one(input: &str) -> usize {
    let possible_commands = input.split("mul(");
    let mut sum: usize = 0;
    for pc in possible_commands {
        let arguments = get_valid_args(pc);
        if let Some((a, b)) = arguments {
            sum += a * b;
        }
    }
    sum
}

fn part_two(input: &str) -> usize {
    let possible_commands = input.split("mul(");
    let mut sum: usize = 0;
    let mut is_enabled = true;
    for pc in possible_commands {
        let arguments = get_valid_args(pc);

        if let (Some((a, b)), true) = (arguments, is_enabled) {
            sum += a * b;
        }

        is_enabled = match (pc.rfind("do()"), pc.rfind("don't()")) {
            (None, None) => is_enabled,
            (None, Some(_)) => false,
            (Some(_), None) => true,
            (Some(en), Some(dis)) => en > dis,
        }
    }
    sum
}

fn get_valid_args(input: &str) -> Option<(usize, usize)> {
    // If it is valid then the stuff inside the brackets is returned

    let (first_arg, idx) = parse_usize(input)?;

    if input[idx..].chars().next()? != ',' {
        return None;
    }

    let second_arg_start = idx + 1; // Skip the comma
    let (second_arg, idx) = parse_usize(&input[second_arg_start..])?;
    let idx = idx + second_arg_start;

    if input[idx..].chars().next()? != ')' {
        return None;
    }

    Some((first_arg, second_arg))
}

fn parse_usize(input: &str) -> Option<(usize, usize)> {
    let mut idx = 0;
    for c in input.chars() {
        if !c.is_numeric() {
            break;
        }

        idx += 1;
    }

    if idx == 0 {
        return None;
    }

    Some((input[..idx].parse().ok()?, idx))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let input = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        assert_eq!(part_one(input), 161);
    }

    #[test]
    fn part_two_works() {
        let input = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        assert_eq!(part_two(input), 48);
    }
}
