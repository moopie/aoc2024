use std::fs::read_to_string;
use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    println!("AOC 2024 day 6!");

    let input = read_to_string("input2.txt").expect("err");

    let count = count_visits(input);

    println!("part 1 count: {}", count);
}

fn count_visits(input: String) -> u32 {
    let map: Vec<Vec<char>> = input.lines()
        .map(|x| x.to_string().trim().chars().collect())
        .filter(|line: &Vec<char>| !line.is_empty())
        .collect();

    for (i, line) in map.iter().enumerate() {
        for (j, b) in line.iter().enumerate() {
            match get_direction(b.clone()) {
                Some(dir) => {
                    return traverse(map, i, j, dir);
                },
                _ => ()
            }
        }
    }
    return 0;
}

fn print_map(map: Vec<Vec<char>>) {
    for ix in map.iter() {
        for jx in ix.iter() {
            print!("{jx}");
        }
        print!("\n");
    }
    print!("\n");
}

fn traverse(map: Vec<Vec<char>>, i: usize, j: usize, dir: Direction) -> u32 {
    let mut map = map;
    let mut i = i;
    let mut j = j;
    let mut dir = dir;
    let mut moves = HashSet::new();

    while i >= 0 || i < map.len() || j >= 0 || j < map[0].len() {
        let (next_i, next_j) = match dir {
            Direction::Up => (i-1, j),
            Direction::Down => (i+1, j),
            Direction::Left => (i, j-1),
            Direction::Right => (i, j+1)
        };

        moves.insert((i, j));

        if next_i < map.len() && next_j < map[0].len() {
            let next = map[next_i][next_j];

            map[i][j] = 'X';

            match next {
                '.' | '^' | 'X' => {
                    i = next_i;
                    j = next_j;
                },
                _ => {
                    dir = change_direction(dir);
                }
            }
        }
        else {
            return moves.len().try_into().unwrap();
        }
    }

    0
}

fn change_direction(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn get_direction(guard: char) -> Option<Direction> {
    match guard {
        '^' => Some(Direction::Up),
        'v' => Some(Direction::Down),
        '<' => Some(Direction::Left),
        '>' => Some(Direction::Right),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_test() {
        let input = r#"
            ....#.....
            .........#
            ..........
            ..#.......
            .......#..
            ..........
            .#..^.....
            ........#.
            #.........
            ......#...
        "#;

        assert_eq!(count_visits(input.to_string()), 41);
    }

    #[test]
    fn zid_test() {
        let input = r#"
        .....
        ..#..
        ...#.
        .....
        ..^..
        "#;

        assert_eq!(count_visits(input.to_string()), 3);
    }
}