fn main() {
    let input = include_str!("../../inputs/day_6.txt");
    let part_one_result = part_one(input);

    println!("Part One: {part_one_result}");
}

#[derive(Copy, Clone, Debug)]
enum Tile {
    Visited,
    Unvisited,
    Blocked,
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn guard_turns(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}
fn part_one(input: &str) -> usize {
    let (mut pos, mut map) = parse_input(input);
    let mut direction = Direction::Up;
    while let Some(_) = move_guard(&mut pos, &mut direction, &mut map) {}
    map.iter()
        .flatten()
        .filter(|t| matches!(t, Tile::Visited))
        .count()
}

fn move_guard(
    pos: &mut (usize, usize),
    dir: &mut Direction,
    map: &mut Vec<Vec<Tile>>,
) -> Option<()> {
    let mut start_pos;
    loop {
        start_pos = pos.clone();
        match dir {
            Direction::Up => start_pos.1 = start_pos.1.checked_sub(1)?,
            Direction::Right => start_pos.0 += 1,
            Direction::Down => start_pos.1 += 1,
            Direction::Left => start_pos.0 = start_pos.0.checked_sub(1)?,
        };

        if start_pos.0 >= map[0].len() || start_pos.1 >= map.len() {
            return None;
        }

        let curr_tile = &mut map[start_pos.1][start_pos.0];
        match curr_tile {
            Tile::Visited => {
                break;
            }
            Tile::Unvisited => {
                *curr_tile = Tile::Visited;
                break;
            }
            Tile::Blocked => {
                // Need to turn again
                *dir = dir.guard_turns();
            }
        };
    }

    *pos = start_pos;
    Some(())
}

fn parse_input(input: &str) -> ((usize, usize), Vec<Vec<Tile>>) {
    let mut starting_pos = (0, 0);
    let map = input
        .lines()
        .enumerate()
        .map(|(row, l)| {
            l.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '.' => Tile::Unvisited,
                    '#' => Tile::Blocked,
                    '^' => {
                        starting_pos = (col, row);
                        Tile::Visited
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    (starting_pos, map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let input = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        assert_eq!(part_one(input), 41);
    }
}
