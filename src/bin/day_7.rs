use itertools::Itertools;
fn main() {
    let input = include_str!("../../inputs/day_7.txt");
    let part_one_result = part_one(input);
    println!("Part One: {part_one_result}");

    let part_two_result = part_two(input);
    println!("Part Two: {part_two_result}");
}

#[derive(Debug, Clone)]
struct Equation {
    test_value: usize,
    parameters: Vec<usize>,
}
impl Equation {
    fn apply_operators(&self, ops: &[Operator]) -> usize {
        self.parameters
            .iter()
            .skip(1)
            .zip(ops)
            .fold(self.parameters[0], |acc, (p, op)| op.apply(acc, *p))
    }
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operator {
    fn apply(&self, lhs: usize, rhs: usize) -> usize {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Multiply => lhs * rhs,
            Operator::Concatenate => format!("{}{}", lhs, rhs).parse().unwrap(),
        }
    }
}

fn part_one(input: &str) -> usize {
    let equations = parse_input(input);

    let operators: [Operator; 2] = [Operator::Add, Operator::Multiply];
    let mut result = 0;
    for e in equations.iter() {
        let curr_result = evaluate_equation(e, &operators);
        result += curr_result;
    }

    result
}
fn part_two(input: &str) -> usize {
    let equations = parse_input(input);

    let operators: [Operator; 3] = [Operator::Add, Operator::Multiply, Operator::Concatenate];
    let mut result = 0;
    for e in equations.iter() {
        let curr_result = evaluate_equation(e, &operators);
        result += curr_result;
    }

    result
}

fn evaluate_equation(eq: &Equation, operators: &[Operator]) -> usize {
    let num_operators = eq.parameters.len() - 1;
    // Get all the different ways that the operators can be arranged (thank you itertools)
    let operator_permutations =
        itertools::repeat_n(operators.to_owned(), num_operators).multi_cartesian_product();

    for operators in operator_permutations {
        let op_result = eq.apply_operators(&operators);
        if op_result == eq.test_value {
            return op_result;
        }
    }

    0
}

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|l| {
            let (test_value, parameters) = l.split_once(':').unwrap();
            let test_value: usize = test_value.parse().unwrap();
            let parameters = parameters
                .trim()
                .split(' ')
                .map(|p| p.parse().unwrap())
                .collect();

            Equation {
                test_value,
                parameters,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let input = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        assert_eq!(part_one(input), 3749);
    }
    #[test]
    fn part_two_works() {
        let input = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        assert_eq!(part_two(input), 11387);
    }
}
