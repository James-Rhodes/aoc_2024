use std::{collections::HashMap, time::Instant};

fn main() {
    let input = include_str!("../../inputs/day_4.txt");
    let now = Instant::now();
    let part_one_result = part_one(input);
    println!("Part One Took: {}", now.elapsed().as_micros());

    println!("Part One: {part_one_result}");

    let now = Instant::now();
    let part_two_result = part_two(input);
    println!("Part Two Took: {}", now.elapsed().as_micros());

    println!("Part Two: {part_two_result}");
}

fn part_one(input: &str) -> usize {
    let word_search: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let word: Vec<char> = "XMAS".chars().collect();

    let x_positions: Vec<(usize, usize)> = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.match_indices(word[0])
                .map(|(pos, _)| (pos, i))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();

    x_positions
        .into_iter()
        .map(|pos| {
            [
                search_direction(&word_search, pos, (0, -1), &word, 1),
                search_direction(&word_search, pos, (1, -1), &word, 1),
                search_direction(&word_search, pos, (1, 0), &word, 1),
                search_direction(&word_search, pos, (1, 1), &word, 1),
                search_direction(&word_search, pos, (0, 1), &word, 1),
                search_direction(&word_search, pos, (-1, 1), &word, 1),
                search_direction(&word_search, pos, (-1, 0), &word, 1),
                search_direction(&word_search, pos, (-1, -1), &word, 1),
            ]
            .into_iter()
            .filter(|b| (*b).is_some())
            .count()
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    let word_search: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let word: Vec<char> = "MAS".chars().collect();

    let m_positions: Vec<(usize, usize)> = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.match_indices(word[0])
                .map(|(pos, _)| (pos, i))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();

    // For each M search the diagonal directions collecting all the positions of the center A
    let mas_centers: Vec<(usize, usize)> = m_positions
        .into_iter()
        .flat_map(|pos| {
            [
                search_direction(&word_search, pos, (1, -1), &word, 1),
                search_direction(&word_search, pos, (1, 1), &word, 1),
                search_direction(&word_search, pos, (-1, 1), &word, 1),
                search_direction(&word_search, pos, (-1, -1), &word, 1),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<(usize, usize)>>()
        })
        .collect();

    mas_centers
        .into_iter()
        .fold(HashMap::<(usize, usize), usize>::new(), |mut hm, pos| {
            // Count how many times each 'A' position occurs
            *hm.entry(pos).or_default() += 1;
            hm
        })
        .iter()
        .filter(|(_pos, cnt)| **cnt == 2) // "MAS" should overlap so when the count is 2
        .count()
}

fn search_direction(
    word_search: &Vec<Vec<char>>,
    incoming_pos: (usize, usize),
    direction: (i32, i32),
    the_word: &Vec<char>,
    search_word_index: usize,
) -> Option<(usize, usize)> {
    // Returns the second to last letter position if the word was found. Otherwise returns None
    //
    // Second to last because part two wants to find all of the occurances of MAS that form an X
    // (X-MAS). So the second to last would be the A which if they are forming an X should be the
    // same for different occurances of MAS. Makes part one a bit lamer but also makes my life
    // easier for part two
    let current_pos = get_new_pos(
        incoming_pos,
        direction,
        word_search.len(),
        word_search[0].len(),
    )?;

    let current_letter = word_search[current_pos.1][current_pos.0];
    let found_letter = current_letter == the_word[search_word_index];

    if !found_letter {
        return None;
    }

    if search_word_index == (the_word.len() - 1) {
        return Some(incoming_pos);
    }

    search_direction(
        word_search,
        current_pos,
        direction,
        the_word,
        search_word_index + 1,
    )
}

const fn get_new_pos(
    incoming_pos: (usize, usize),
    direction: (i32, i32),
    x_bound: usize,
    y_bound: usize,
) -> Option<(usize, usize)> {
    let new_x = incoming_pos.0 as i32 + direction.0;
    let new_y = incoming_pos.1 as i32 + direction.1;

    if new_x >= x_bound as i32 || new_x < 0 || new_y >= y_bound as i32 || new_y < 0 {
        return None;
    }
    Some((new_x as usize, new_y as usize))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let input = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(part_one(input), 18);
    }

    #[test]
    fn part_two_works() {
        let input = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(part_two(input), 9);
    }
}
