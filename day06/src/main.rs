enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    println!("AOC 2024 day 6!");
}

fn count_visits(input: &str) -> u32 {
    let map = input.trim();

    for row in map.lines() {
        for col in row.chars() {

        }
    }
    return 0;
}

fn get_direction(char guard) -> Option<Direction> {
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

        assert_eq!(count_visits(input), 41);
    }
}