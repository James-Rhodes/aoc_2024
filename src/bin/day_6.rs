use std::collections::HashSet;

fn main() {
    let input = include_str!("../../inputs/day_6.txt");
    let part_one_result = part_one(input);

    println!("Part One: {part_one_result}");

    let part_two_result = part_two(input);

    println!("Part Two: {part_two_result}");
}

#[derive(Copy, Clone, Debug)]
enum Tile {
    Visited,
    Unvisited,
    Blocked,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
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
    while move_guard(&mut pos, &mut direction, &mut map).is_some() {}
    map.iter()
        .flatten()
        .filter(|t| matches!(t, Tile::Visited))
        .count()
}

fn part_two(input: &str) -> usize {
    let (pos, map) = parse_input(input);
    let direction = Direction::Up;

    let mut num_loops = 0;

    // Brute force for the win
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if block_caused_loop(pos, direction, &map, (x, y)) {
                num_loops += 1;
            }
        }
    }
    num_loops
}

fn move_guard(pos: &mut (usize, usize), dir: &mut Direction, map: &mut [Vec<Tile>]) -> Option<()> {
    let mut start_pos;
    loop {
        start_pos = *pos;
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

enum GuardStatus {
    Looping,
    Patrolling,
}

fn move_guard_cached(
    pos: &mut (usize, usize),
    dir: &mut Direction,
    map: &mut [Vec<Tile>],
    cache: &mut HashSet<((usize, usize), Direction)>,
) -> Option<GuardStatus> {
    let mut start_pos;
    loop {
        start_pos = *pos;
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
    if cache.contains(&(start_pos, *dir)) {
        // Same direction and same position so must be looping
        return Some(GuardStatus::Looping);
    } else {
        cache.insert((start_pos, *dir));
    }

    Some(GuardStatus::Patrolling)
}

fn block_caused_loop(
    pos: (usize, usize),
    direction: Direction,
    map: &[Vec<Tile>],
    block_pos: (usize, usize),
) -> bool {
    let mut pos = pos;
    let mut map = map.to_owned();
    if block_pos == pos {
        return false;
    }

    map[block_pos.1][block_pos.0] = Tile::Blocked;
    let mut dir = direction;
    let mut cache = HashSet::new();
    while let Some(guard_status) = move_guard_cached(&mut pos, &mut dir, &mut map, &mut cache) {
        if matches!(guard_status, GuardStatus::Looping) {
            return true;
        }
    }
    false
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

    #[test]
    fn part_two_works() {
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

        assert_eq!(part_two(input), 6);
    }
}
